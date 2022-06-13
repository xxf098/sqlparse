# A SQL Parser and Formatter for Rust

## Example
To parse a sql statement:

```rust
    use sqlparse::{Parser};

    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";

    // grouping
    let tokens = p.parse(sql);
    println!("{:?}", tokens);
    // without grouping
    let tokens = p.parse_no_grouping(sql);
    println!("{:?}", tokens);

```

To format a simple SELECT statement:

```rust
    use sqlparse::{FormatOption, Formatter};

    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC";

    let mut f = Formatter::default();
    let mut options = FormatOption::default();
    options.reindent = true;
    options.indent_width = 2;
    options.indent_char = " ";
    options.reindent_aligned = true;

    
    let formatted_sql = f.format(sql, &mut options);
    println!("{}", formatted_sql);

```

outputs
```sql
SELECT a,
       b,
       123,
       myfunc(b)
  FROM table_1
 WHERE a > b
   AND b < 100
 ORDER BY a DESC
```

To format multiple statements:

```rust
  use sqlparse::{FormatOption, Formatter};
  let sql = "SELECT firstname, lastname, email FROM employees WHERE employeeNumber = 1056; UPDATE employees SET email = 'mary.patterson@classicmodelcars.com' WHERE employeeNumber = 1056;";
  let mut formatter = FormatOption::default_reindent();
  formatter.reindent_aligned = true;
  let formatted_sql = f.format(sql, &mut formatter);
  println!("{}", formatted_sql);

```

outputs
```sql
SELECT firstname,
       lastname,
       email
  FROM employees
 WHERE employeeNumber = 1056;
UPDATE employees
   SET email = 'mary.patterson@classicmodelcars.com'
 WHERE employeeNumber = 1056;
```

see more on ./examples folder