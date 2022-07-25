
//! SQL Parser and Formatter for Rust
//!
//! Example code, see more on Github:
//!
//!
//! ```
//! use sqlparse::{FormatOption, Formatter};
//!
//! let sql = "SELECT a, b, 123, myfunc(b) \
//!            FROM table_1 \
//!            WHERE a > b AND b < 100 \
//!            ORDER BY a DESC";
//!
//! let mut f = Formatter::default();
//! let mut formatter = FormatOption::default();
//! formatter.reindent = true;
//! formatter.reindent_aligned = true;
//! 
//! let formatted_sql = f.format(sql, &mut formatter);
//! println!("{}", formatted_sql);
//!
//! ```
//! Output:
//! 
//! ```sql
//! SELECT a,
//!        b,
//!        123,
//!        myfunc(b)
//!   FROM table_1
//!  WHERE a > b
//!    AND b < 100
//!  ORDER BY a DESC
//! ```

mod engine;
mod lexer;
mod keywords;
mod tokens;
mod formatter;
mod filters;
mod trie;


pub use tokens::TokenType;
pub use lexer::{Token, TokenList};
pub use formatter::{FormatOption};
pub use engine::grouping::group_tokenlist;
pub use trie::Trie;

/// parse sql
pub struct Parser {
    stack: engine::FilterStack,
}

impl Default for Parser {
    fn default() -> Self {
        Self { stack: engine::FilterStack::new() }
    }
}

// TODO: add option
impl Parser {

    pub fn new() -> Self {
        Self { stack: engine::FilterStack::new() }
    }

    /// parse single sql statement
    pub fn parse(&self, sql: &str) -> Vec<Token> {
        self.stack.run(sql, true)
    }

    /// parse multiple sql statements
    pub fn parse_multi(&self, sql: &str) -> Vec<Vec<Token>> {
        self.stack.run_multi(sql, true)
    }

    pub fn parse_no_grouping(&self, sql: &str) -> Vec<Token> {
        self.stack.run(sql, false)
    }

    pub fn parse_multi_no_grouping(&self, sql: &str) -> Vec<Vec<Token>> {
        self.stack.run_multi(sql, false)
    }
}

/// parse sql into tokens,
/// only for test
pub fn parse(sql: &str) -> Vec<Token> {
    let stack = engine::FilterStack::new();
    stack.run(sql, true)
}

/// parse multiple sqls into tokens,
/// only for test
pub fn parse_multi(sql: &str) -> Vec<Vec<Token>> {
    let stack = engine::FilterStack::new();
    stack.run_multi(sql, true)
}

/// parse sql into grouped tokens,
/// only for test
pub fn parse_no_grouping(sql: &str) -> Vec<Token> {
    let stack = engine::FilterStack::new();
    stack.run(sql, false)
}

/// format sql with multiple options
pub struct Formatter {
    stack: engine::FilterStack,
}


impl Default for Formatter {
    fn default() -> Self {
        Self { stack: engine::FilterStack::new() }
    }
}

impl Formatter {

    pub fn new(stack: engine::FilterStack) -> Self {
        Self { stack }
    }

    /// do not use this function repeatly
    pub fn format(&mut self, mut sql: &str, options: &mut formatter::FormatOption) -> String {
        formatter::validate_options(options);
        formatter::build_filter_stack(&mut self.stack, options);
        if options.strip_whitespace { sql = sql.trim(); };
        let tokens = self.stack.format(sql, options.grouping);
        tokens.iter().map(|token| token.iter().map(|t| t.value.as_str()).collect::<String>()).collect::<Vec<_>>().join("\n")
    }

    pub fn build_filters(&mut self, options: &mut formatter::FormatOption) {
        formatter::validate_options(options);
        formatter::build_filter_stack(&mut self.stack, options);
    }

    pub fn format_sql(&mut self, sql: &str, options: &formatter::FormatOption) -> String {
        let tokens = self.stack.format(sql, options.grouping);
        tokens.iter().map(|token| token.iter().map(|t| t.value.as_str()).collect::<String>()).collect::<Vec<_>>().join("\n")
    }
}

/// format sql to string,
/// only for test
pub fn format(mut sql: &str, options: &mut formatter::FormatOption) -> String {
    let mut stack = engine::FilterStack::new();
    formatter::validate_options(options);
    formatter::build_filter_stack(&mut stack, options);
    if options.strip_whitespace { sql = sql.trim(); };
    let tokens = stack.format(sql, options.grouping);
    // for token in &tokens{
    //     println!("{:?}", token);
    // }
    tokens.iter().map(|token| token.iter().map(|t| t.value.as_str()).collect::<String>()).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_parse() {
        let sql = "select * from users where id > 1 order by id;";
        let tokens = parse(sql);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_parse_identifier() {
        let sql = "select * from sch.users;";
        let tokens = parse(sql);
        // let tokens = parse_no_grouping(sql);
        for token in tokens {
            println!("{:?} {}", token.typ, token.value);
        }
    }

    #[test]
    fn test_parser1() {
        let sql= "SELECT article, MAX(price) AS price FROM shop GROUP BY article ORDER BY article;";
        let p = Parser::default();
        let now = Instant::now();
        let _tokens = p.parse(sql);
        let elapsed = now.elapsed();
        println!("elapsed: {}ms", elapsed.as_millis());
    }


    #[test]
    fn test_parser2() {
        let sql= "s";
        let p = Parser::default();
        let tokens = p.parse(sql);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].typ, TokenType::Identifier);
        println!("{:?}", tokens);
    }

    #[test]
    fn test_parser3() {
        let sql= "SELECT COUNT(CustomerID), Country FROM Customers GROUP BY Country HAVING COUNT(CustomerID) > 5 ORDER BY COUNT(CustomerID) DESC;";
        let p = Parser::default();
        let now = Instant::now();
        let _tokens = p.parse(sql);
        let elapsed = now.elapsed();
        println!("elapsed: {}ms", elapsed.as_millis());
    }

}
