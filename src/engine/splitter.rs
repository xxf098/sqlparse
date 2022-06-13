use crate::lexer::{Token};
use crate::tokens::TokenType;

const EOS_TTYPE: [TokenType; 2] = [TokenType::Whitespace, TokenType::CommentSingle];

#[derive(Default)]
pub struct StatementSplitter {
    in_declare: bool,
    is_create: bool,
    consume_ws: bool,
    begin_depth: usize,
    level: isize,
    tokens: Vec<Token>
}

impl StatementSplitter {

    fn reset(&mut self) {
        self.in_declare = false;
        self.is_create = false;
        self.consume_ws = false;
        self.begin_depth = 0;
        self.level = 0;
        self.tokens = vec![];
    }

    fn change_splitlevel(&mut self, token: &Token) -> isize {
        if token.typ == TokenType::Punctuation && token.value == "(" {
            return 1;
        } else if token.typ == TokenType::Punctuation && token.value == ")" {
            return -1;
        } else if !token.is_keyword() {
            return 0;
        }

        let unified = token.value.to_uppercase();
        if token.typ == TokenType::KeywordDDL && unified.starts_with("CREATE") {
            self.is_create = true;
            return 0
        }

        if unified == "DECLARE" && self.is_create && self.begin_depth == 0 {
            self.in_declare = true;
            return 1
        }
        if unified == "BEGIN" {
            self.begin_depth += 1;
            if self.is_create {
                return 1
            }
            return 0
        }
        if unified == "END" {
            self.begin_depth = if self.begin_depth > 1 { self.begin_depth -1 } else { 0 };
            return -1
        }
        if (unified == "IF" || unified == "FOR" || unified == "WHILE" || unified == "CASE") && self.is_create && self.begin_depth > 0 {
            return 1
        }
        if unified == "END IF" || unified == "END FOR" || unified == "END WHILE" {
            return -1
        }
        0
    }

    pub fn process(&mut self, tokens: Vec<Token>) -> Vec<Vec<Token>> {
        let mut stmts = vec![];
        for token in tokens.into_iter() {
            if self.consume_ws && !EOS_TTYPE.contains(&token.typ) {
                let stmt_tokens = std::mem::replace(&mut self.tokens, vec![]);
                stmts.push(stmt_tokens);
                self.reset();
            }

            self.level += self.change_splitlevel(&token);
            if self.level <= 0 && token.typ == TokenType::Punctuation && token.value == ";"  {
                self.consume_ws = true
            }
            self.tokens.push(token);
        }
        if self.tokens.len() > 0 && self.tokens.iter().find(|t| t.typ != TokenType::Whitespace).is_some() {
            let stmt_tokens = std::mem::replace(&mut self.tokens, vec![]);
            stmts.push(stmt_tokens);
        }
        stmts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_no_grouping;

    #[test]
    fn test_parse_splitter() {
        let sql = "select 'one'; select 'two'; select 'two';";
        let tokens = parse_no_grouping(sql);
        let mut splitter = StatementSplitter::default();
        let stmts = splitter.process(tokens);
        assert_eq!(stmts.len(), 3);
    }

    #[test]
    fn test_parse_splitter_function() {
        let sql = r#"   CREATE FUNCTION a(x VARCHAR(20)) RETURNS VARCHAR(20)
        BEGIN
         DECLARE y VARCHAR(20);
         RETURN x;
        END;
        SELECT * FROM a.b;"#;
        let tokens = parse_no_grouping(sql);
        let mut splitter = StatementSplitter::default();
        let stmts = splitter.process(tokens);
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn test_parse_splitter_function1() {
        let sql = r#"   CREATE FUNCTION a(x VARCHAR(20)) RETURNS VARCHAR(20)
        BEGIN
         DECLARE y VARCHAR(20);
         IF (1 = 1) THEN
         SET x = y;
         END IF;
         RETURN x;
        END;
        SELECT * FROM a.b;"#;
        let tokens = parse_no_grouping(sql);
        let mut splitter = StatementSplitter::default();
        let stmts = splitter.process(tokens);
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn test_parse_splitter_multi() {
        let sql = r#"CREATE OR REPLACE RULE ruled_tab_2rules AS ON INSERT
TO public.ruled_tab
DO instead (
select 1;
select 2;
);"#;
        let tokens = parse_no_grouping(sql);
        let mut splitter = StatementSplitter::default();
        let stmts = splitter.process(tokens);
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_parse_splitting_at_and_backticks() {
        let sql = "grant foo to user1@`myhost`; grant bar to user1@`myhost`;";
        let tokens = parse_no_grouping(sql);
        let mut splitter = StatementSplitter::default();
        let stmts = splitter.process(tokens);
        assert_eq!(stmts.len(), 2);
    }

}
