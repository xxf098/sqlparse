
use sqlparse::{Token, parse_multi};

fn to_string(tokens: &[Token]) -> String {
    tokens.iter().map(|t| t.value.clone()).collect::<Vec<_>>().join("")
}


#[test]
fn test_split_semicolon() {
    let sql = "select * from foo;select * from foo where bar = 'foo;bar';";
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 2);
    let sql1 = to_string(&stmts[0]);
    assert_eq!(sql1, "select * from foo;");
    let sql1 = to_string(&stmts[1]);
    assert_eq!(sql1, "select * from foo where bar = 'foo;bar';");
}

