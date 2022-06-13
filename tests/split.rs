
use sqlparse::{Token, parse_multi};

fn to_string(tokens: &[Token]) -> String {
    tokens.iter().map(|t| t.value.as_str()).collect::<Vec<_>>().join("")
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


#[test]
fn test_split_backslash() {
    let sql = r#"select '\\'; select '\''; select '\\\'';"#;
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 3);
}


#[test]
fn test_split_create_function() {
    let sql = r#"CREATE OR REPLACE FUNCTION foo(
    p_in1 VARCHAR
    , p_in2 INTEGER
) RETURNS INTEGER AS

    DECLARE
    v_foo INTEGER;
    BEGIN
        SELECT *
        FROM foo
        INTO v_foo;
        RETURN v_foo.id;
    END;
"#;
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 1);
    let sql1 = to_string(&stmts[0]);
    assert_eq!(sql, sql1);
}

#[test]
fn test_split_create_function2() {
    let sql = r#"CREATE OR REPLACE FUNCTION update_something() RETURNS void AS
$body$
BEGIN
    raise notice 'foo';
END;
$body$
LANGUAGE 'plpgsql' VOLATILE CALLED ON NULL INPUT SECURITY INVOKER;
"#;
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 1);
    // let sql1 = to_string(&stmts[0]);
    // assert_eq!(sql, sql1);
}

#[test]
fn test_split_dashcomments() {
    let sql = r#"select * from user;
--select * from host;
select * from user;
select * -- foo;
from foo;"#;
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 3);
    let sql1 = stmts.iter().map(|t| to_string(t)).collect::<Vec<_>>().join("");
    assert_eq!(sql, sql1);
}

#[test]
fn test_split_dashcomments_eol() {
    let sqls = vec![
        "select foo; -- comment\n", 
        "select foo; -- comment\r",
        "select foo; -- comment\r\n",
        "select foo; -- comment"];
    for sql in sqls {
        let stmts = parse_multi(sql);
        assert_eq!(stmts.len(), 1);
    }
}

#[test]
fn test_split_begintag() {
    let sql = "begin;
update foo
       set bar = 1;
commit;";
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 3);
}

#[test]
fn test_split_begintag2() {
    let sql = "CREATE TRIGGER IF NOT EXISTS remove_if_it_was_the_last_file_link
-- Delete the direntry when is removed it's last static link
    AFTER DELETE ON links
    WHEN NOT EXISTS
    (
        SELECT * FROM links
        WHERE child_entry = OLD.child_entry
        LIMIT 1
    )
BEGIN
    DELETE FROM dir_entries
    WHERE dir_entries.inode = OLD.child_entry;
END;";
    let stmts = parse_multi(sql);
    assert_eq!(stmts.len(), 1);
}
