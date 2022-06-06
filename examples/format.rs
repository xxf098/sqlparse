use sqlparse::{FormatOption, format, Formatter};

fn main() {
    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";
    let mut formatter = FormatOption::default();
    formatter.reindent = true;
    let formatted_sql = format(sql, &mut formatter);
    println!("{}", formatted_sql);

    let mut f = Formatter::default();
    let mut formatter = FormatOption::default();
    formatter.reindent = true;
    let formatted_sql = f.format(sql, &mut formatter);
    println!("{}", formatted_sql);
}
