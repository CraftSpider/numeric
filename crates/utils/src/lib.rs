#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod intern;
#[cfg(feature = "alloc")]
mod linked;
mod macros;

#[cfg(feature = "alloc")]
pub use intern::Interner;

#[cfg(feature = "std")]
#[cfg(test)]
pub(crate) mod tests {
    extern crate std;
    use std::thread;

    pub const THREAD_COUNT: usize = {
        #[cfg(miri)]
        {
            100
        }
        #[cfg(not(miri))]
        {
            100
        }
    };

    pub fn run_threaded<T, C, F>(ctx: C, f: F) -> T
    where
        T: Sync,
        C: FnOnce() -> T,
        F: Fn(&T, usize) + Send + Sync + Copy,
    {
        let list = ctx();
        thread::scope(|scope| {
            let list = &list;
            let mut joins = Vec::new();
            for i in 0..THREAD_COUNT {
                joins.push(scope.spawn(move || f(list, i)));
            }
            for j in joins {
                j.join().unwrap()
            }
        });
        list
    }
}
