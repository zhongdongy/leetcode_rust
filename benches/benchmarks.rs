mod b000_0xx;
use b000_0xx::bench_b000_0xx;
use criterion::criterion_main;

criterion_main!(bench_b000_0xx);
