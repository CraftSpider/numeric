//! Benchmark measurements that track allocation-related statistics.
//!
//! Note that these measurements can only track allocations that use the Rust global allocator.
//! As such they can't be used to benchmark FFI code, or anything that uses custom allocators.

use crate::trace_alloc::{Checkpoint, TracingAlloc};
use criterion::measurement::{Measurement, ValueFormatter};
use criterion::Throughput;
use std::alloc::System;

// TODO: Fix `TracingAlloc`
// #[global_allocator]
static GLOBAL_ALLOC: TracingAlloc<System> = TracingAlloc::new(System);

struct AllocFormatter;

impl ValueFormatter for AllocFormatter {
    fn format_value(&self, value: f64) -> String {
        let mut values = [value];
        let unit = self.scale_values(value, &mut values);
        format!("{:>6.0} {}", values[0], unit)
    }

    fn scale_values(&self, _: f64, _: &mut [f64]) -> &'static str {
        "allocations"
    }

    fn scale_throughputs(&self, _: f64, _: &Throughput, _: &mut [f64]) -> &'static str {
        "allocations"
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        "allocations"
    }
}

struct BytesFormatter;

fn scale_base(typical_value: f64) -> (&'static str, f64) {
    let base = 1024.0f64;
    if typical_value < base {
        ("b", 1.0)
    } else if typical_value < base.powi(2) {
        ("kb", base)
    } else if typical_value < base.powi(3) {
        ("mb", base.powi(2))
    } else if typical_value < base.powi(4) {
        ("gb", base.powi(3))
    } else {
        ("tb", base.powi(4))
    }
}

impl ValueFormatter for BytesFormatter {
    fn format_value(&self, value: f64) -> String {
        let mut values = [value];
        let unit = self.scale_values(value, &mut values);
        format!("{:>6.0} {}", values[0], unit)
    }

    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        let (out, scale) = scale_base(typical_value);
        values.iter_mut().for_each(|b| *b /= scale);
        out
    }

    fn scale_throughputs(
        &self,
        typical_value: f64,
        _: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        let (out, scale) = scale_base(typical_value);
        values.iter_mut().for_each(|b| *b /= scale);
        out
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        "b"
    }
}

/// Measurement that tracks total allocations occuring during a benchmark
pub struct Allocations;

impl Measurement for Allocations {
    type Intermediate = usize;
    type Value = usize;

    fn start(&self) -> Self::Intermediate {
        GLOBAL_ALLOC.with_stats(|stats| stats.total())
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        let end = GLOBAL_ALLOC.with_stats(|stats| stats.total());
        end - start
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &AllocFormatter
    }
}

/// Measurement that tracks total bytes allocated during a benchmark
pub struct BytesAllocated;

impl Measurement for BytesAllocated {
    type Intermediate = usize;
    type Value = usize;

    fn start(&self) -> Self::Intermediate {
        GLOBAL_ALLOC.with_stats(|stats| stats.total_bytes())
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        let end = GLOBAL_ALLOC.with_stats(|stats| stats.total_bytes());
        end - start
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &BytesFormatter
    }
}

/// Measurement that tracks peak live bytes allocated during a benchmark
pub struct PeakMemory;

impl Measurement for PeakMemory {
    type Intermediate = Checkpoint;
    type Value = usize;

    fn start(&self) -> Self::Intermediate {
        GLOBAL_ALLOC.with_stats(|stats| stats.checkpoint())
    }

    fn end(&self, start: Self::Intermediate) -> Self::Value {
        GLOBAL_ALLOC.with_stats(|stats| {
            let end = stats.checkpoint();
            stats
                .points_between(start, end)
                .iter()
                .map(|v| v.live_bytes())
                .max()
                .unwrap()
        })
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        v1 + v2
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        *value as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &BytesFormatter
    }
}
