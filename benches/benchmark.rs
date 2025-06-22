use criterion::{criterion_group, criterion_main, Criterion};
use order_processor::runs;
use pprof::criterion::{Output, PProfProfiler};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run_with_inferring", |b| {
        b.iter(|| {
            #[cfg(all(feature = "error_handling", not(feature = "no_inferring")))]
            runs::run_inferring();
            #[cfg(all(feature = "error_handling", feature = "no_inferring"))]
            runs::run_no_inferring();
            #[cfg(not(feature = "error_handling"))]
            runs::run_optimized();
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::new(30,0))
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);
