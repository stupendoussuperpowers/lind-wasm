#[cfg(feature = "lind_perf")]
pub mod enabled {
    use lind_perf::Counter;

    pub static CLOSE_SYSCALL: Counter = Counter::new("rawposix::close_syscall");
    pub static GETEUID_SYSCALL: Counter = Counter::new("rawposix::geteuid_syscall");

    pub static ALL_COUNTERS: &[&Counter] = &[&CLOSE_SYSCALL, &GETEUID_SYSCALL];

    pub fn enable_all() {
        lind_perf::enable_all(ALL_COUNTERS);
    }

    pub fn disable_all() {
        lind_perf::disable_all(ALL_COUNTERS);
    }

    pub fn reset_all() {
        lind_perf::reset_all(ALL_COUNTERS);
    }

    pub fn report() {
        lind_perf::report(ALL_COUNTERS);
    }
}

#[cfg(not(feature = "lind_perf"))]
pub mod enabled {
    pub fn enable_all() {}
    pub fn disable_all() {}
    pub fn reset_all() {}
    pub fn report() {}
}
