#[cfg(feature = "lind_perf")]
pub mod enabled {
    use lind_perf::Counter;

    pub static MAKE_SYSCALL: Counter = Counter::new("threei::make_syscall");
    pub static MAKE_SYSCALL_CHECK_HANDLER_TABLE: Counter =
        Counter::new("threei::make_syscall::check_handler_table");
    pub static CALL_GRATE_FUNC: Counter = Counter::new("threei::_call_grate_func");
    pub static CALL_GRATE_FUNC_GET_RUNTIME_TRAMPOLINE: Counter =
        Counter::new("threei::_call_grate_func::get_runtime_trampoline");

    pub static ALL_COUNTERS: &[&Counter] = &[
        &MAKE_SYSCALL,
        &MAKE_SYSCALL_CHECK_HANDLER_TABLE,
        &CALL_GRATE_FUNC,
        &CALL_GRATE_FUNC_GET_RUNTIME_TRAMPOLINE,
    ];

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
