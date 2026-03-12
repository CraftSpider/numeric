//! Utilities for benchmarks of other numeric crates.

use criterion::Criterion;
#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

#[cfg(feature = "alloc-measures")]
pub mod alloc_measure;
pub mod trace_alloc;

/// Create the [`Criterion`] profiler instance. This automatically includes a `PProf` profiler if
/// compiling on unix targets.
pub fn make_criterion() -> Criterion {
    let c = Criterion::default();
    #[cfg(unix)]
    let c = c.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    c.configure_from_args()
}
