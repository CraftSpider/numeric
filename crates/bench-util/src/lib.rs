use criterion::Criterion;
#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

pub fn make_criterion() -> Criterion {
    let c = Criterion::default();
    #[cfg(unix)]
    let c = c.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    c.configure_from_args()
}
