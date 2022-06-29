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


criterion_group!(
    benches,
    simple_query,
    complex_query,
);
criterion_main!(benches);