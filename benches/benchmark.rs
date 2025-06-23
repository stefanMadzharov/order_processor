use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use order_processor::runs;

#[cfg(all(feature = "error_handling", feature = "inferring"))]
fn run_inferring(c: &mut Criterion) {
    c.bench_function("run_inferring", |b| {
        b.iter(|| {
            runs::run_inferring();
        });
    });
}

#[cfg(all(feature = "error_handling", not(feature = "inferring")))]
fn run_no_inferring(c: &mut Criterion) {
    c.bench_function("run_no_inferring", |b| {
        b.iter(|| {
            runs::run_no_inferring();
        });
    });
}

#[cfg(not(feature = "error_handling"))]
fn run_optimized(c: &mut Criterion) {
    c.bench_function("run_optimized", |b| {
        b.iter(|| {
            runs::run_optimized();
        });
    });
}

#[cfg(all(feature = "error_handling", feature = "inferring"))]
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(6,500));
    targets = run_inferring
}

#[cfg(all(feature = "error_handling", not(feature = "inferring")))]
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(6,500));
    targets = run_no_inferring
}

#[cfg(not(feature = "error_handling"))]
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(7,0));
    targets = run_optimized
}

criterion_main!(benches);
