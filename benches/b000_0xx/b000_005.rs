use criterion::{black_box, Criterion};
use leetcode_rust::cases::c000_0xx::*;
use leetcode_rust::problems::p000_0xx::*;

pub fn longest_palindrome(c: &mut Criterion) {
    let cases = c000_005::use_cases();
    c.bench_function("Longest_Palindrome", |b| {
        b.iter(|| {
            cases.iter().for_each(move |c| {
                p000_005::longest_palindrome(black_box(c.input().clone()));
                ()
            });
        })
    });
}
