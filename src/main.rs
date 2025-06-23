use order_processor::runs;

fn main() {
    #[cfg(all(feature = "error_handling", feature = "inferring"))]
    runs::run_inferring();
    #[cfg(all(feature = "error_handling", not(feature = "inferring")))]
    runs::run_no_inferring();
    #[cfg(not(feature = "error_handling"))]
    runs::run_optimized();
}
