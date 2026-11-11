//! lind-perf: scope timers for runtime hot paths that need no plumbing at the call site.
//!
//! Instrument a function:
//!
//! ```ignore
//! #[lind_perf::timed]
//! fn _call_grate_func(/* ... */) -> i32 { /* ... */ }
//!
//! #[lind_perf::timed(key = syscall_num)]   // break down by syscall number
//! pub fn make_syscall(/* ... */ syscall_num: u64, /* ... */) -> i32 { /* ... */ }
//! ```
//!
//! or a block:
//!
//! ```ignore
//! let g = lind_perf::scope!("threei::rawposix_dispatch", key = syscall_num);
//! let r = func(/* ... */);
//! drop(g);
//! ```
//!
//! Every callsite owns a hidden `static` counter that registers itself the first time it runs.
//! There is no per-crate counter list, no central registry, and nothing for the binary to
//! enumerate. Without the `enabled` feature every macro expands to a zero-sized value.
//!
//! Runtime control is through environment variables, read once:
//!
//! | var              | meaning                                                         |
//! |------------------|-----------------------------------------------------------------|
//! | `LIND_PERF`      | `clock` (or `1`) or `tsc`. Unset means record nothing.          |
//! | `LIND_PERF_ONLY` | comma-separated names to enable (`a::` / `a*` = prefix); unset = all |
//! | `LIND_PERF_OUT`  | append the report to this file instead of stderr.               |
//!
//! The report is written automatically at process exit (`atexit`), or when you call
//! [`report`]. See README.md for the format and the driver script.

#[cfg_attr(not(feature = "enabled"), allow(dead_code))]
mod timers;
pub use timers::TimerKind;

pub use lind_perf_macros::timed;

#[cfg(feature = "enabled")]
mod enabled;
#[cfg(feature = "enabled")]
pub use enabled::{init, report, reset, Guard};

/// True when timers are compiled in.
pub const ENABLED: bool = cfg!(feature = "enabled");

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "enabled")]
    pub use crate::enabled::{Bucket, Counter, KEYED_BUCKETS};
}

/// Start timing until the returned guard is dropped.
///
/// `scope!(name)`, `scope!(name, key = expr)`, `scope!(name, cond = expr)` or
/// `scope!(name, key = expr, cond = expr)`. `name` must be a `&'static str` constant expression.
/// `key` is cast `as u64` and selects a per-key bucket (for example a syscall number). When `cond`
/// is false the call is not timed and the counter is not touched.
#[cfg(feature = "enabled")]
#[macro_export]
macro_rules! scope {
    ($name:expr, cond = $cond:expr) => {
        if $cond { $crate::scope!($name) } else { $crate::Guard::off() }
    };
    ($name:expr, key = $key:expr, cond = $cond:expr) => {
        if $cond { $crate::scope!($name, key = $key) } else { $crate::Guard::off() }
    };
    ($name:expr) => {{
        static __LIND_PERF_BUCKETS: [$crate::__private::Bucket; 1] =
            [$crate::__private::Bucket::new()];
        static __LIND_PERF_COUNTER: $crate::__private::Counter =
            $crate::__private::Counter::new($name, false, &__LIND_PERF_BUCKETS);
        __LIND_PERF_COUNTER.guard(0)
    }};
    ($name:expr, key = $key:expr) => {{
        static __LIND_PERF_BUCKETS: [$crate::__private::Bucket;
            $crate::__private::KEYED_BUCKETS] =
            [const { $crate::__private::Bucket::new() }; $crate::__private::KEYED_BUCKETS];
        static __LIND_PERF_COUNTER: $crate::__private::Counter =
            $crate::__private::Counter::new($name, true, &__LIND_PERF_BUCKETS);
        __LIND_PERF_COUNTER.guard(($key) as u64)
    }};
}

/// Start timing until the returned guard is dropped. Compiled out: `enabled` is off.
#[cfg(not(feature = "enabled"))]
#[macro_export]
macro_rules! scope {
    ($name:expr) => {
        $crate::Guard
    };
    ($name:expr, key = $key:expr) => {
        $crate::Guard
    };
    ($name:expr, cond = $cond:expr) => {
        $crate::Guard
    };
    ($name:expr, key = $key:expr, cond = $cond:expr) => {
        $crate::Guard
    };
}

/// Zero-sized guard used when timers are compiled out.
#[cfg(not(feature = "enabled"))]
pub struct Guard;

#[cfg(not(feature = "enabled"))]
pub fn report() {}

#[cfg(not(feature = "enabled"))]
pub fn reset() {}

#[cfg(not(feature = "enabled"))]
pub fn init() {}
