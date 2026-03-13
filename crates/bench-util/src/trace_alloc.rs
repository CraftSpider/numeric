//! Implementation of a global allocator that support tracing of allocation statistics

use atomic_cell::AtomicCell;
use std::alloc::{GlobalAlloc, Layout};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicU64;
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::thread::ThreadId;
use thread_local::ThreadLocal;

/// Allocation tracing checkpoint, allows referencing allocation data as of a given point
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Checkpoint(usize);

/// Allocation data tracked from a specific point
pub struct TracePoint {
    total: usize,
    total_bytes: usize,
    live: usize,
    live_bytes: usize,
}

impl TracePoint {
    fn new(stats: &TracingStats) -> TracePoint {
        TracePoint {
            total: stats.total,
            total_bytes: stats.total_bytes,
            live: stats.live(),
            live_bytes: stats.live_bytes(),
        }
    }

    /// Get the total number of allocations at this point
    pub fn total(&self) -> usize {
        self.total
    }

    /// Get the total number of allocated bytes at this point
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Get the number of live allocations at this point
    pub fn live(&self) -> usize {
        self.live
    }

    /// Get the number of live allocated bytes at this point
    pub fn live_bytes(&self) -> usize {
        self.live_bytes
    }
}

/// Statistics collected by the tracing allocator
pub struct TracingStats {
    total: usize,
    total_bytes: usize,
    live: BTreeMap<usize, Layout>,
    series: Vec<TracePoint>,
}

impl TracingStats {
    const fn new() -> TracingStats {
        TracingStats {
            total: 0,
            total_bytes: 0,
            live: BTreeMap::new(),
            series: Vec::new(),
        }
    }

    fn add_alloc(&mut self, ptr: *const u8, layout: Layout) {
        self.total += 1;
        self.total_bytes += layout.size();
        self.live.insert(ptr.addr(), layout);
        self.series.push(TracePoint::new(self))
    }

    fn remove_alloc(&mut self, ptr: *const u8, _: Layout) {
        self.live.remove(&ptr.addr());
        self.series.push(TracePoint::new(self))
    }

    fn move_alloc(&mut self, old: *const u8, new: *const u8, layout: Layout, new_size: usize) {
        if new_size > layout.size() {
            self.total_bytes += new_size - layout.size();
        }
        self.live.remove(&old.addr());
        self.live.insert(
            new.addr(),
            Layout::from_size_align(new_size, layout.align()).unwrap(),
        );
        self.series.push(TracePoint::new(self))
    }

    /// Get total allocations seen
    pub fn total(&self) -> usize {
        self.total
    }

    /// Get total number of bytes allocated. Re-allocations only count bytes beyond the original
    /// size.
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Get total number of live allocations
    pub fn live(&self) -> usize {
        self.live.len()
    }

    /// Get total number of live allocated bytes
    pub fn live_bytes(&self) -> usize {
        self.live.values().map(|l| l.size()).sum()
    }

    /// Get information about current live allocations
    pub fn live_allocs(&self) -> &BTreeMap<usize, Layout> {
        &self.live
    }

    /// Get a checkpoint at the current location
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.series.len())
    }

    /// Get all allocation points between two checkpoints
    pub fn points_between(&self, start: Checkpoint, end: Checkpoint) -> &[TracePoint] {
        &self.series[start.0..end.0]
    }

    /// Get all allocation points between two checkpoints
    pub fn points_after(&self, start: Checkpoint) -> &[TracePoint] {
        &self.series[start.0..]
    }
}

/// Global allocator that tracks allocations, while deferring actual operations to an underlying
/// allocator
pub struct TracingAlloc<A> {
    alloc: A,
    stats: Mutex<TracingStats>,
    trace: ThreadLocal<Cell<bool>>,
    allocating: LazyLock<AtomicCell<Option<ThreadId>, AtomicU64>>,
}

impl<A> TracingAlloc<A> {
    /// Create a new tracing allocator
    pub const fn new(alloc: A) -> TracingAlloc<A> {
        TracingAlloc {
            alloc,
            stats: Mutex::new(TracingStats::new()),
            trace: ThreadLocal::new(),
            allocating: LazyLock::new(|| AtomicCell::new(None)),
        }
    }

    /// Get the statistics collected so far.
    ///
    /// Note that any allocations that occur inside the closure won't be tracked as part of the
    /// statistics.
    ///
    /// # Panics
    ///
    /// If called while inside the closure passed to `with_stats`.
    pub fn with_stats<T, F: FnOnce(&TracingStats) -> T>(&self, f: F) -> T {
        self.maybe_trace(|stats| f(stats.expect("with_stats should not be called reentrantly")))
    }

    fn maybe_trace<T, F: FnOnce(Option<&mut TracingStats>) -> T>(&self, f: F) -> T {
        // Mark this thread as potentially recursively allocating
        //  This is for before we have a thread-local, so we globally lock until we're done
        let cur_id = thread::current().id();
        while let Err(id) = self.allocating.compare_exchange::<true>(None, Some(cur_id)) {
            if id == Some(cur_id) {
                return f(None);
            }
        }
        let trace = self.trace.get_or(|| Cell::new(true));
        self.allocating.store(None);

        // Mark this thread as potentially recursively allocating
        //  Now that we have a thread-local, we only care about re-entrance in just this thread
        let tl_trace = trace.replace(false);
        let out = {
            let mut guard;
            let stats = if tl_trace {
                guard = self.stats.lock().unwrap();
                Some(&mut *guard)
            } else {
                None
            };
            f(stats)
        };
        trace.replace(tl_trace);
        out
    }
}

// SAFETY: `TracingAlloc` carefully fulfills the safety guidelines, and uses `Mutex` carefully to
// avoid re-entrance issues.
unsafe impl<A: GlobalAlloc> GlobalAlloc for TracingAlloc<A> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.maybe_trace(|stats| {
            let out = self.alloc.alloc(layout);
            if let Some(stats) = stats {
                stats.add_alloc(out, layout);
            }
            out
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.maybe_trace(|stats| {
            if let Some(stats) = stats {
                stats.remove_alloc(ptr, layout);
            }
            self.alloc.dealloc(ptr, layout)
        })
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.maybe_trace(|stats| {
            let out = self.alloc.alloc_zeroed(layout);
            if let Some(stats) = stats {
                stats.add_alloc(out, layout)
            }
            out
        })
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.maybe_trace(|stats| {
            let out = self.alloc.realloc(ptr, layout, new_size);
            if let Some(stats) = stats {
                stats.move_alloc(ptr, out, layout, new_size)
            }
            out
        })
    }
}
