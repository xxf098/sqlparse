# A SQL Parser and Formatter for Rust

## Example
To format a simple SELECT statement:

```rust
    use sqlparse::{FormatOption, Formatter};

    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC";

    let mut f = Formatter::default();
    let mut formatter = FormatOption::default();
    formatter.reindent = true;
    formatter.reindent_aligned = true;
    
    let formatted_sql = f.format(sql, &mut formatter);
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