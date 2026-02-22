#[cfg(feature = "lind_perf")]
pub mod enabled {
    use lind_perf::Counter;

    pub static READ_WASM_OR_CWASM: Counter = Counter::new("lind_boot::read_wasm_or_cwasm");
    pub static LOAD_MAIN_MODULE: Counter = Counter::new("lind_boot::load_main_module");
    pub static INVOKE_FUNC: Counter = Counter::new("lind_boot::invoke_func");
    pub static GRATE_CALLBACK_TRAMPOLINE: Counter =
        Counter::new("lind_boot::grate_callback_trampoline");
    pub static TRAMPOLINE_GET_VMCTX: Counter =
        Counter::new("lind_boot::trampoline::get_vmctx");
    pub static TRAMPOLINE_CALLER_WITH: Counter =
        Counter::new("lind_boot::trampoline::Caller::with");
    pub static TRAMPOLINE_GET_PASS_FPTR_TO_WT: Counter =
        Counter::new("lind_boot::trampoline::get_pass_fptr_to_wt");
    pub static TRAMPOLINE_TYPED_DISPATCH_CALL: Counter =
        Counter::new("lind_boot::trampoline::typed_dispatch_call");

    pub static ALL_COUNTERS: &[&Counter] = &[
        &READ_WASM_OR_CWASM,
        &LOAD_MAIN_MODULE,
        &INVOKE_FUNC,
        &GRATE_CALLBACK_TRAMPOLINE,
        &TRAMPOLINE_GET_VMCTX,
        &TRAMPOLINE_CALLER_WITH,
        &TRAMPOLINE_GET_PASS_FPTR_TO_WT,
        &TRAMPOLINE_TYPED_DISPATCH_CALL,
    ];

    pub fn enable_all() {
        let _ = lind_perf::set_timer_source_from_env("LIND_PERF_TIMER");
        lind_perf::enable_all(ALL_COUNTERS);
        threei::perf::enabled::enable_all();
        rawposix::perf::enabled::enable_all();
        fdtables::perf::enabled::enable_all();
        wasmtime_lind_common::perf::enabled::enable_all();
    }

    pub fn disable_all() {
        lind_perf::disable_all(ALL_COUNTERS);
        threei::perf::enabled::disable_all();
        rawposix::perf::enabled::disable_all();
        fdtables::perf::enabled::disable_all();
        wasmtime_lind_common::perf::enabled::disable_all();
    }

    pub fn reset_all() {
        lind_perf::reset_all(ALL_COUNTERS);
        threei::perf::enabled::reset_all();
        rawposix::perf::enabled::reset_all();
        fdtables::perf::enabled::reset_all();
        wasmtime_lind_common::perf::enabled::reset_all();
    }

    pub fn report() {
        lind_perf::report(ALL_COUNTERS);
        wasmtime_lind_common::perf::enabled::report();
        threei::perf::enabled::report();
        rawposix::perf::enabled::report();
        fdtables::perf::enabled::report();
    }

    pub fn set_timer_source(source: &str) -> bool {
        let lower = source.trim().to_ascii_lowercase();
        let source = match lower.as_str() {
            "rdtsc" | "tsc" | "cycles" | "0" => lind_perf::TimerSource::Rdtsc,
            "clock" | "clock_gettime" | "clockgettime" | "monotonic" | "ns" | "1" => {
                lind_perf::TimerSource::ClockGettime
            }
            _ => return false,
        };
        lind_perf::set_timer_source(source);
        true
    }
}

#[cfg(not(feature = "lind_perf"))]
pub mod enabled {
    pub fn enable_all() {}
    pub fn disable_all() {}
    pub fn reset_all() {}
    pub fn report() {}
    pub fn set_timer_source(_source: &str) -> bool {
        false
    }
}
