use criterion::Criterion;
use order_processor::runs;
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

pub fn main() {
    let mut c = Criterion::default()
        .profile_time(Some(Duration::new(2, 0)))
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));

    #[cfg(all(feature = "error_handling", feature = "inferring"))]
    c.bench_function("run_inferring", |b| {
        b.iter(|| {
            runs::run_inferring();
        });
    });

    #[cfg(all(feature = "error_handling", not(feature = "inferring")))]
    c.bench_function("run_no_inferring", |b| {
        b.iter(|| {
            runs::run_no_inferring();
        });
    });

    #[cfg(not(feature = "error_handling"))]
    c.bench_function("run_optimized", |b| {
        b.iter(|| {
            runs::run_optimized();
        });
    });

    c.final_summary();
}
