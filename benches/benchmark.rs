#[macro_use]
extern crate criterion;

use criterion::{Criterion, BatchSize};
use criterion::black_box;

fn criterion_benchmark(c: &mut Criterion) {
}

criterion_group!(benches, criterion_benchma