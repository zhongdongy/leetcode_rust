mod b000_005;
mod b000_006;
use criterion::{Criterion,criterion_group};

criterion_group! {
  name=bench_b000_0xx;
  config=Criterion::default();
  targets=b000_005::longest_palindrome, b000_006::zigzag_conversion
}