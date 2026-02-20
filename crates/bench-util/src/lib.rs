//! Utilities for benchmarks of other numeric crates.

use criterion::Criterion;

/// Create the [`Criterion`] profiler instance. This automatically includes a `PProf` profiler if
/// compiling on unix targets.
pub fn make_criterion() -> Criterion {
    let c = Criterion::default();
    // #[cfg(unix)]
    // let c = c.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    c.configure_from_args()
}
