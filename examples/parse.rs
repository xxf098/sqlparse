use sqlparse::{Parser};


fn main() {
    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";
    
    let p = Parser::default();
    // grouping
    let tokens = p.parse(sql);
    println!("{:?}", tokens);
    // no grouping
    let tokens = p.parse_no_grouping(sql);
    println!("{:?}", tokens);
}
