use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlparse::{FormatOption, Formatter};

fn simple_query(c: &mut Criterion) {
    let sql = "SELECT * FROM my_table WHERE id = 1";
    let mut f = Formatter::default();
    let mut formatter = FormatOption::default();
    formatter.reindent = true;
    formatter.reindent_aligned = true;
    f.build_filters(&mut formatter);
    c.bench_function("simple query", |b| {
        b.iter(|| {
            f.format_sql(black_box(sql), black_box(&formatter));
        })
    });
}


criterion_group!(
    benches,
    simple_query,
);
criterion_main!(benches);