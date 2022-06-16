use sqlparse::{FormatOption, format};

#[test]
fn test_strip_comments_single() {
    let sql = "select *-- statement starts here\nfrom foo";
    let mut formatter = FormatOption::default();
    formatter.strip_comments = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\nfrom foo");
    let sql = "select * -- statement starts here\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\nfrom foo");
    let sql = "select-- foo\nfrom -- bar\nwhere";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select\nfrom\nwhere");
    let sql = "select *-- statement starts here\n\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\nfrom foo");
    let sql = "select * from foo-- statement starts here\nwhere";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select * from foo\nwhere");
    let sql = "select a-- statement starts here\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select a\nfrom foo");
    let sql = "--comment\nselect a-- statement starts here\nfrom foo--comment\nf";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select a\nfrom foo\nf");
}

#[test]
fn test_strip_comments_multi() {
    let sql = "/* sql starts here */\nselect";
    let mut formatter = FormatOption::default();
    formatter.strip_comments = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select");
    let sql = "/* sql starts here */ select";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select");
    let sql = "/*\n * sql starts here\n */\nselect";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select");
    let sql = "select (/* sql starts here */ select 2)";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select (select 2)");
    let sql = "select (/* sql /* starts here */ select 2)";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select (select 2)");
}

#[test]
fn test_strip_comments_preserves_linebreak() {
    let sql = "select * -- a comment\r\nfrom foo";
    let mut formatter = FormatOption::default();
    formatter.strip_comments = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\r\nfrom foo");
    let sql = "select * -- a comment\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\nfrom foo");
    let sql = "select * -- a comment\rfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\rfrom foo");
    let sql = "select * -- a comment\r\n\r\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\r\nfrom foo");
    let sql = "select * -- a comment\n\nfrom foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *\nfrom foo");
}

#[test]
fn test_strip_ws1() {
    let sql = "select\n* from      foo\n\twhere  ( 1 = 2 )\n";
    let mut formatter = FormatOption::default();
    formatter.strip_whitespace = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select * from foo where (1 = 2)");
    let sql = "select -- foo\nfrom    bar\n";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select -- foo\nfrom bar");
}

#[test]
fn test_notransform_of_quoted_crlf() {
    let sql = "SELECT some_column LIKE 'value\r'";
    let mut formatter = FormatOption::default();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT some_column LIKE 'value\r'");
    let sql = "SELECT some_column LIKE 'value\r'\r\nWHERE id = 1\n";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT some_column LIKE 'value\r'\r\nWHERE id = 1\n");
    let sql = "SELECT some_column LIKE 'value\\'\r' WHERE id = 1\r";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT some_column LIKE 'value\\'\r' WHERE id = 1\r");
    let sql = "SELECT some_column LIKE 'value\\\\\\'\r' WHERE id = 1\r\n";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT some_column LIKE 'value\\\\\\'\r' WHERE id = 1\r\n");
}

#[test]
fn test_preserve_ws() {
    let sql = "select\n* /* foo */  from bar ";
    let mut formatter = FormatOption::default();
    formatter.strip_whitespace = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select * /* foo */ from bar");
}

#[test]
fn test_aligned_stmts() {
    let sql = "select foo; select bar";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select foo;\nselect bar");
    let sql = "select foo";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, sql);
    let sql = "select foo; -- test\n select bar";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select foo; -- test\n\nselect bar");
}


#[test]
fn test_aligned_basic() {
    let sql = r#"
    select a, b as bb,c from table
    join (select a * 2 as a from new_table) other
    on table.a = other.a
    where c is true
    and b between 3 and 4
    or d is 'blue'
    limit 10        
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select a,",
        "       b as bb,",
        "       c",
        "  from table",
        "  join (",
        "        select a * 2 as a",
        "          from new_table",
        "       ) other",
        "    on table.a = other.a",
        " where c is true",
        "   and b between 3 and 4",
        "    or d is 'blue'",
        " limit 10"
    ].join("\n"));
}

#[test]
fn test_aligned_joins() {
    let sql = r#"
    select * from a
    join b on a.one = b.one
    left join c on c.two = a.two and c.three = a.three
    full outer join d on d.three = a.three
    cross join e on e.four = a.four
    join f using (one, two, three)    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select *",
        "  from a",
        "  join b",
        "    on a.one = b.one",
        "  left join c",
        "    on c.two = a.two",
        "   and c.three = a.three",
        "  full outer join d",
        "    on d.three = a.three",
        " cross join e",
        "    on e.four = a.four",
        "  join f using (one, two, three)"
    ].join("\n"));
}

#[test]
fn test_aligned_case_statement() {
    let sql = r#"
    select a,
    case when a = 0
    then 1
    when bb = 1 then 1
    when c = 2 then 2
    else 0 end as d,
    extra_col
    from table
    where c is true
    and b between 3 and 4    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select a,",
        "       case when a = 0  then 1",
        "            when bb = 1 then 1",
        "            when c = 2  then 2",
        "            else 0",
        "             end as d,",
        "       extra_col",
        "  from table",
        " where c is true",
        "   and b between 3 and 4",        
    ].join("\n"));
}

#[test]
fn test_aligned_case_statement_with_between() {
    let sql = r#"
    select a,
    case when a = 0
    then 1
    when bb = 1 then 1
    when c = 2 then 2
    when d between 3 and 5 then 3
    else 0 end as d,
    extra_col
    from table
    where c is true
    and b between 3 and 4    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select a,",
        "       case when a = 0             then 1",
        "            when bb = 1            then 1",
        "            when c = 2             then 2",
        "            when d between 3 and 5 then 3",
        "            else 0",
        "             end as d,",
        "       extra_col",
        "  from table",
        " where c is true",
        "   and b between 3 and 4",        
    ].join("\n"));
}


#[test]
fn test_aligned_group_by() {
    let sql = r#"
    select a, b, c, sum(x) as sum_x, count(y) as cnt_y
    from table
    group by a,b,c
    having sum(x) > 1
    and count(y) > 5
    order by 3,2,1    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select a,",
        "       b,",
        "       c,",
        "       sum(x) as sum_x,",
        "       count(y) as cnt_y",
        "  from table",
        " group by a,",
        "          b,",
        "          c",
        "having sum(x) > 1",
        "   and count(y) > 5",
        " order by 3,",
        "          2,",
        "          1"            
    ].join("\n"));      
}


#[test]
fn test_aligned_group_by_subquery() {
    let sql = r#"
    select *, sum_b + 2 as mod_sum
    from (
      select a, sum(b) as sum_b
      from table
      group by a,z)
    order by 1,2    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{:?}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select *,",
        "       sum_b + 2 as mod_sum",
        "  from (",
        "        select a,",
        "               sum(b) as sum_b",
        "          from table",
        "         group by a,",
        "                  z",
        "       )",
        " order by 1,",
        "          2",
    ].join("\n"));      
}

#[test]
fn test_aligned_window_functions() {
    let sql = r#"
    select a,
    SUM(a) OVER (PARTITION BY b ORDER BY c ROWS
    BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) as sum_a,
    ROW_NUMBER() OVER
    (PARTITION BY b, c ORDER BY d DESC) as row_num
    from table    
    "#;
    let mut formatter = FormatOption::default();
    formatter.reindent_aligned = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql.split("\n").count(), 4);

}

#[test]
fn test_space_around_basic() {
    let sql = "select a+b as d from table where (c-d)%2= 1 and e> 3.0/4 and z^2 <100;";
    let mut formatter = FormatOption::default();
    formatter.use_space_around_operators = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, "select a + b as d from table where (c - d) % 2 = 1 and e > 3.0 / 4 and z ^ 2 < 100;");
}

#[test]
fn test_space_around_bool() {
    let sql = "select * from table where a &&b or c||d";
    let mut formatter = FormatOption::default();
    formatter.use_space_around_operators = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, "select * from table where a && b or c || d");
}

#[test]
fn test_space_around_nested() {
    let sql = "select *, case when a-b then c end from table";
    let mut formatter = FormatOption::default();
    formatter.use_space_around_operators = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select *, case when a - b then c end from table");
}

#[test]
fn test_space_around_wildcard_vs_mult() {
    let sql = "select a*b-c from table";
    let mut formatter = FormatOption::default();
    formatter.use_space_around_operators = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select a * b - c from table");
}


#[test]
fn test_reindent_identifier_list_with_functions() {
    let sql = "select 'abc' as foo, coalesce(col1, col2)||col3 as bar, col3 from my_table";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select 'abc' as foo,",
        "       coalesce(col1, col2)||col3 as bar,",
        "       col3",
        "from my_table"
    ].join("\n"));
}



#[test]
fn test_reindent_identifier_list_comment_first() {
    let sql = "select foo, bar, baz from table where foo in (1, 2,3)";
    let mut formatter = FormatOption::default_reindent();
    formatter.comma_first = true;
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select foo",
        "     , bar",
        "     , baz",
        "from table",
        "where foo in (1",
        "            , 2",
        "            , 3)"
    ].join("\n"));
}


#[test]
fn test_reindent_identifier_list_with_wrap_after() {
    let sql = "select foo, bar, baz from table1, table2 where 1 = 2";
    let mut formatter = FormatOption::default_reindent();
    formatter.wrap_after = 14;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select foo, bar,",
        "       baz",
        "from table1, table2",
        "where 1 = 2"
    ].join("\n"));
}


#[test]
fn test_reindent_identifier_list() {
    let sql = "select foo, bar, baz from table1, table2 where 1 = 2";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select foo,",
        "       bar,",
        "       baz",
        "from table1,",
        "     table2",
        "where 1 = 2"
    ].join("\n"));
    let sql = "select a.*, b.id from a, b";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select a.*,",
        "       b.id",
        "from a,",
        "     b"
    ].join("\n"));
}

#[test]
fn test_reindent_case() {
    let sql = "case when foo = 1 then 2 when foo = 3 then 4 else 5 end";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "case",
        "    when foo = 1 then 2",
        "    when foo = 3 then 4",
        "    else 5",
        "end"
    ].join("\n"));
}

#[test]
fn test_reindent_case2() {
    let sql = "case(foo) when bar = 1 then 2 else 3 end";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "case(foo)",
        "    when bar = 1 then 2",
        "    else 3",
        "end"
    ].join("\n"));

}


#[test]
fn test_reindent_insert_values_comma_first() {
    let sql = "insert into foo values (1, 2)";
    let mut formatter = FormatOption::default_reindent();
    formatter.comma_first = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo",
        "values (1, 2)",
    ].join("\n"));

    let sql = "insert into foo values (1, 2), (3, 4), (5, 6)";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo",
        "values (1, 2)",
        "     , (3, 4)",
        "     , (5, 6)",
    ].join("\n"));

    let sql = "insert into foo(a, b) values (1, 2), (3, 4), (5, 6)";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo(a, b)",
        "values (1, 2)",
        "     , (3, 4)",
        "     , (5, 6)",
    ].join("\n"));
}


#[test]
fn test_reindent_insert_values() {
    let sql = "insert into foo values (1, 2)";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo",
        "values (1, 2)",
    ].join("\n"));

    let sql = "insert into foo values (1, 2), (3, 4), (5, 6)";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo",
        "values (1, 2),",
        "       (3, 4),",
        "       (5, 6)",
    ].join("\n"));

    let sql = "insert into foo(a, b) values (1, 2), (3, 4), (5, 6)";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "insert into foo(a, b)",
        "values (1, 2),",
        "       (3, 4),",
        "       (5, 6)",
    ].join("\n"));

}


#[test]
fn test_strip_ws() {
    let sql = "select     * from  users where  id  = 1;";
    let mut formatter = FormatOption::default();
    formatter.strip_whitespace = true;
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "select * from users where id = 1;");
}

#[test]
fn test_reindent_keywords() {
    let sql = "select * from foo union select * from bar;";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select *", 
        "from foo", 
        "union", 
        "select *", 
        "from bar;"].join("\n"))
}

#[test]
fn test_reindent_keywords_between() {
    let sql = "and foo between 1 and 2 and bar = 3";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "and foo between 1 and 2",
        "and bar = 3",
    ].join("\n"))
}

#[test]
fn test_reindent_where() {
    let sql = "select * from foo where bar = 1 and baz = 2 or bzz = 3;";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select *",
        "from foo",
        "where bar = 1",
        "  and baz = 2",
        "  or bzz = 3;",
    ].join("\n"))
}

#[test]
fn test_reindent_parenthesis() {
    let sql = "select count(*) from (select * from foo);";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    // println!("{}", formatted_sql);
    assert_eq!(formatted_sql, vec![
        "select count(*)",
        "from",
        "  (select *",
        "   from foo);",
    ].join("\n"))
}

#[test]
fn test_reindent_join() {
    let sql = "select * from foo join bar on 1 = 2";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select *",
        "from foo",
        "join bar on 1 = 2",
    ].join("\n"));
    let sql = "select * from foo inner join bar on 1 = 2";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select *",
        "from foo",
        "inner join bar on 1 = 2",
    ].join("\n"));
    let sql = "select * from foo left outer join bar on 1 = 2";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select *",
        "from foo",
        "left outer join bar on 1 = 2",
    ].join("\n"));
    let sql = "select * from foo straight_join bar on 1 = 2";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, vec![
        "select *",
        "from foo",
        "straight_join bar on 1 = 2",
    ].join("\n"));
}


#[test]
fn test_format() {
    let sql = "select * from users limit 10";
    let mut formatter = FormatOption::default();
    formatter.keyword_case = "upper";
    formatter.identifier_case = "upper";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT * FROM USERS LIMIT 10");
    let sql = "select * from \"t\".\"users\" limit 10";
    let formatted_sql = format(sql, &mut formatter);
    assert_eq!(formatted_sql, "SELECT * FROM \"t\".\"users\" LIMIT 10");
}