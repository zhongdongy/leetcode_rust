use criterion::{black_box, Criterion};
use leetcode_rust::cases::c000_0xx::*;
use leetcode_rust::problems::p000_0xx::*;

pub fn zigzag_conversion(c: &mut Criterion) {
    c.bench_function("ZigZag_Conversion", |b| {
        b.iter(|| {
            c000_006::use_cases().iter().for_each(move |c| {
                p000_006::zigzag_conversion(
                    black_box(c.input().clone()),
                    black_box(c.params[0]),
                    black_box(None),
                );
                ()
            });
        })
    });
}
