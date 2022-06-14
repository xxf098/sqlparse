use regex::Regex;
use super::{StmtFilter, TokenListFilter};
use crate::lexer::{Token, TokenList};
use crate::tokens::{TokenType};

pub struct StripCommentsFilter {
    newline_reg: Regex,
}

impl Default for StripCommentsFilter {
    fn default() -> Self {
        Self { newline_reg: Regex::new(r"((\r|\n)+) *$").unwrap() }
    }
}

impl StripCommentsFilter {

    fn get_next_comment(&self, token_list: &mut TokenList, start: usize) -> Option<usize> {
        let ttypes = vec![TokenType::CommentSingle, TokenType::CommentMultiline];
        token_list.token_next_by(&ttypes, None, start)
    }

    fn get_insert_token(&self, token: &Token) -> Token {
        let caps = self.newline_reg.captures(&token.value);
        if let Some(caps) = caps {
            if let Some(cap) = caps.get(1).map(|c| c.as_str()) {
                return Token::new(TokenType::Newline, cap)
            } 
        }
        Token::new(TokenType::Whitespace, " ")
    }

    fn process_internal(&self, token_list: &mut TokenList) {
        let mut tidx = self.get_next_comment(token_list, 0);
        while let Some(idx) = tidx {
            let token = token_list.token_idx(Some(idx)).unwrap();
            let pidx = token_list.token_prev(idx, false);
            let ptoken = token_list.token_idx(pidx);
            let nidx = token_list.token_next(idx, false);
            let ntoken = token_list.token_idx(nidx);
            let insert_token = self.get_insert_token(token);
            if ptoken.is_none() || ntoken.is_none() ||
                 ptoken.map(|p| p.is_whitespace()).unwrap_or(false) || ptoken.map(|p| p.typ == TokenType::Punctuation && p.value == "(").unwrap_or(false) ||
                 ntoken.map(|p| p.is_whitespace()).unwrap_or(false) || ntoken.map(|p| p.typ == TokenType::Punctuation && p.value == ")").unwrap_or(false)  {
                    if ptoken.is_some() && !ptoken.map(|p| p.typ == TokenType::Punctuation && p.value == "(").unwrap() {
                        token_list.insert_after(idx, insert_token, false)
                    }
                    token_list.tokens.remove(idx);
            } else {
                token_list.tokens[idx] = insert_token;
            }
            tidx = self.get_next_comment(token_list, idx + 1);
        }
    }
}

impl TokenListFilter for StripCommentsFilter {

    fn process(&mut self, token_list: &mut TokenList) {
        for token in token_list.tokens.iter_mut() {
            if token.is_group() { self.process_internal(&mut token.children); }
        }
        self.process_internal(token_list)
    }
}

pub struct StripWhitespaceFilter { }

impl StripWhitespaceFilter {

    fn stripws(tokens: &mut Vec<Token>) {
        StripWhitespaceFilter::stripws_default(tokens);
        StripWhitespaceFilter::stripws_newline(tokens);
    }

    fn stripws_default(tokens: &mut Vec<Token>) {
        let mut last_was_ws = false;
        let mut is_first_char = true;
        let n = tokens.len();
        for (i, token) in tokens.iter_mut().enumerate() {
            if token.is_whitespace() {
                token.value = if last_was_ws || is_first_char || i+1 == n { "".to_string() } else { " ".to_string() };
            }
            last_was_ws = token.is_whitespace();
            is_first_char = false;
        }
    }

    // remove whitespace after newline
    fn stripws_newline(tokens: &mut Vec<Token>) {
        let mut idx = 0;
        while idx < tokens.len() {
            let token = &tokens[idx];
            if token.typ != TokenType::Newline {
                idx += 1;
                continue
            }
            let next_idx = idx+1;
            while next_idx < tokens.len() {
                let token_next = &tokens[next_idx];
                if !token_next.is_whitespace() {
                    break
                }
                tokens.remove(next_idx);
            }
            idx += 1;
        }
    }

    fn stripws_parenthesis(token: &mut Token) {
        if token.typ != TokenType::Parenthesis {
            return
        }
        if token.children.token_idx(Some(1)).map(|t| t.is_whitespace()).unwrap_or(false) {
            token.children.tokens.remove(1);
        }
        let token_len = token.children.len();
        if token_len> 2 && token.children.token_idx(Some(token_len-2)).map(|t| t.is_whitespace()).unwrap_or(false) {
            token.children.tokens.remove(token_len-2);
        }
    }

}

impl StmtFilter for StripWhitespaceFilter {

    fn process(&self, tokens: &mut Vec<Token>) {
        for token in tokens.iter_mut() {
            if token.is_group() {
                Self::stripws_parenthesis(token);
                self.process(&mut token.children.tokens);
                token.update_value();
            }
        }
        Self::stripws(tokens);
    }
}

pub struct SpacesAroundOperatorsFilter{}

impl SpacesAroundOperatorsFilter {

    fn process_internal(&mut self, token_list: &mut TokenList) {
        let types = vec![TokenType::Operator, TokenType::OperatorComparison];
        let mut tidx = token_list.token_next_by(&types, None, 0);
        while let Some(mut idx) = tidx {
            let nidx = token_list.token_next(idx+1, false);
            if let Some(token_next) = token_list.token_idx(nidx) {
                if token_next.typ != TokenType::Whitespace {
                    token_list.insert_after(idx, Token::new(TokenType::Whitespace, " "), true);
                } 
            }

            let pidx = token_list.token_prev(idx, false);
            if let Some(token_prev) = token_list.token_idx(pidx) {
                if token_prev.typ != TokenType::Whitespace {
                    token_list.insert_before(idx, Token::new(TokenType::Whitespace, " "));
                }
                idx += 1;
            }
            
            tidx = token_list.token_next_by(&types, None, idx+1);
        }
    }
}

impl TokenListFilter for SpacesAroundOperatorsFilter {

    fn process(&mut self, token_list: &mut TokenList) {
        self.process_internal(token_list);
        for token in token_list.tokens.iter_mut() {
            if token.is_group() {
                // let before = token.children.len();
                self.process(&mut token.children);
                // println!("before {}, after {}", before, token.children.len());
                token.update_value();
            }
        }
    }
}


// trim space before newline
pub struct StripBeforeNewline{}

impl StmtFilter for StripBeforeNewline {

    fn process(&self, tokens: &mut Vec<Token>) {
        let mut remove_indexes = vec![];
        let mut is_before_white = false;
        for (i, token) in tokens.iter_mut().enumerate() {         
            if token.is_group() {
                self.process(&mut token.children.tokens);
            }
            if is_before_white && token.value.starts_with("\n") && i > 0 {
                remove_indexes.push(i-1);
            }
            is_before_white = if token.is_group() {
                // check last token of group is whitespace
                if let Some(t) = token.children.tokens.last() { t.is_whitespace() } else { false }
             } else { token.is_whitespace() };
        }
        let mut remove_count = 0;
        remove_indexes.iter().for_each(|idx| {
            let token = &mut tokens[idx-remove_count];
            let l = token.children.len();
            if l > 0 {
                token.children.tokens.remove(l-1);
                token.update_value();
            } else { 
                tokens.remove(idx-remove_count);
                remove_count += 1;
            }
        });
    }

} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_insert_token() {
        let filter = StripCommentsFilter::default();
        let token = Token::new(TokenType::CommentSingle, "-- comment \n\n ");
        let t = filter.get_insert_token(&token);
        assert_eq!(t.typ, TokenType::Newline);
        assert_eq!(t.value, "\n\n");

        let token = Token::new(TokenType::CommentSingle, "-- comment ");
        let t = filter.get_insert_token(&token);
        assert_eq!(t.typ, TokenType::Whitespace);
        assert_eq!(t.value, " ");

    }
}