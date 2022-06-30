use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlparse::{FormatOption, Formatter};

fn simple_query(c: &mut Criterion) {
    let sql = "SELECT * FROM my_table WHERE id = 1";
    let mut f = Formatter::default();
    let mut formatter = FormatOption::default_reindent();
    formatter.reindent_aligned = true;
    f.build_filters(&mut formatter);
    c.bench_function("simple query", |b| {
        b.iter(|| {
            f.format_sql(black_box(sql), black_box(&formatter));
        })
    });
}

fn complex_query(c: &mut Criterion) {
    let sql = "SELECT t1.id, t1.name, t1.title, t1.description, t2.mothers_maiden_name, t2.first_girlfriend\nFROM my_table t1 LEFT JOIN other_table t2 ON t1.id = t2.other_id WHERE t2.order BETWEEN  17 AND 30";
    let mut f = Formatter::default();
    let mut formatter = FormatOption::default_reindent();
    formatter.reindent_aligned = true;
    f.build_filters(&mut formatter);
    c.bench_function("complex query", |b| {
        b.iter(|| {
            f.format_sql(black_box(sql), black_box(&formatter));
        })
    });
}


fn multiple_statements_query(c: &mut Criterion) {
    const SIZE: usize = 1000;

    pub struct UserData {
        pub id: i64,
        pub first_name: String,
        pub last_name: String,
        pub address: String,
        pub email: String,
        pub phone: String,
    }

    fn sample() -> UserData {
        UserData {
            id: -1,
            first_name: "FIRST_NAME".to_string(),
            last_name: "LAST_NAME".to_string(),
            address: "SOME_ADDRESS".to_string(),
            email: "email@example.com".to_string(),
            phone: "9999999999".to_string(),
        }
    }

    fn to_insert_params(user_data: &UserData) -> String {
        format!(
            r#"('{}', '{}', '{}', '{}', '{}')"#,
            user_data.first_name,
            user_data.last_name,
            user_data.address,
            user_data.email,
            user_data.phone,
        )
    }

    static INSERT_QUERY: &str = "
INSERT INTO user_data
(first_name, last_name, address, phone, email)
VALUES
";

    fn generate_insert_query() -> String {
        let mut query_str = String::with_capacity(1_000_000);
        query_str.push_str(INSERT_QUERY);
        let mut is_first = true;
        let sample_data = sample();
        for _ in 0..SIZE {
            if is_first {
                is_first = false;
            } else {
                query_str.push(',');
            }
            let params = to_insert_params(&sample_data);
            query_str.push_str(&params);
        }
        query_str.push(';');
        query_str
    }

    let sql = generate_insert_query();
    let mut f = Formatter::default();
    let mut formatter = FormatOption::default_reindent();
    formatter.reindent_aligned = true;
    f.build_filters(&mut formatter);
    c.bench_function("issue 633", |b| {
        b.iter(|| {
            f.format_sql(black_box(&sql), black_box(&formatter));
        })
    });
}

criterion_group!(
    benches,
    simple_query,
    complex_query,
    multiple_statements_query,
);
criterion_main!(benches);