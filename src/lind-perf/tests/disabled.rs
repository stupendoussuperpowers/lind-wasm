//! Without `enabled`, the same callsites must compile and do nothing.
#![cfg(not(feature = "enabled"))]

#[lind_perf::timed(key = n)]
fn keyed(n: u64) -> u64 {
    n + 1
}

#[test]
fn compiles_to_nothing() {
    assert!(!lind_perf::ENABLED);
    assert_eq!(keyed(1), 2);
    let g = lind_perf::scope!("x", key = 5);
    drop(g);
    assert_eq!(std::mem::size_of_val(&lind_perf::scope!("y")), 0);
    lind_perf::init();
    lind_perf::report();
}
