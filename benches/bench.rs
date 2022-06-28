use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlparse::{FormatOption, format};


fn simple_query(c: &mut Criterion) {
    let sql = "SELECT * FROM my_table WHERE id = 1";
    let mut formatter = FormatOption::default();
    formatter.reindent = true;
    c.bench_function("simple query", |b| {
        b.iter(|| {
            format(
                black_box(sql),
                black_box(&mut formatter),
            )
        })
    });
}


criterion_group!(
    benches,
    simple_query,
);
criterion_main!(benches);