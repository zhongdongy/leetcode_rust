use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode_rust::problems::p000_0xx::p000_010::is_match;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Match long pattern", |b| {
        b.iter(|| {
            is_match(
                black_box("aaaaaaaaaaaaaaaaaaab".to_string()),
                black_box("a*a*a*a*a*a*a*a*a*a*a*a*a*a*a*".to_string()),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
