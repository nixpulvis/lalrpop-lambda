#[macro_use]
extern crate criterion;
extern crate lalrpop_lambda;

use criterion::Criterion;
use lalrpop_lambda::Expression;

fn compare_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs("native addition", |b, &n| {
        b.iter(|| {
            n + n
        })
    }, &[0,1,2,4,8,16,32]);

    c.bench_function_over_inputs("λ-expression addition", |b, &n| {
        b.iter(|| {
            let e = Expression::from(*n);
            u64::from(e.clone() + e)
        })
    }, &[0,1,2,4,8,16,32]);
}

criterion_group!(benches, compare_benchmark);
criterion_main!(benches);
