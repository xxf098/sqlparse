use sqlparse::{FormatOption, format, Formatter};

fn main() {
    let sql = "SELECT firstname, lastname, email FROM employees WHERE employeeNumber = 1056; UPDATE employees SET email = 'mary.patterson@classicmodelcars.com' WHERE employeeNumber = 1056;";
    let mut formatter = FormatOption::default_reindent();
    let formatted_sql = format(sql, &mut formatter);
    println!("{}", formatted_sql);

    let mut f = Formatter::default();
    let mut formatter = FormatOption::default_reindent();
    formatter.reindent_aligned = true;
    let formatted_sql = f.format(sql, &mut formatter);
    println!("{}", formatted_sql);
}
