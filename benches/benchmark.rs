use order_processor::runs;

fn main() {
    let guard = pprof::ProfilerGuard::new(100).unwrap();

    #[cfg(all(feature = "error_handling", not(feature = "no_inferring")))]
    runs::run_inferring();
    #[cfg(all(feature = "error_handling", feature = "no_inferring"))]
    runs::run_no_inferring();
    #[cfg(not(feature = "error_handling"))]
    runs::run_optimized();
    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
        println!("{:?}", report.timing.duration);
    }
}
