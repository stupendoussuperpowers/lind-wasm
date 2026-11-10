//! Run with `cargo test --features enabled`. Config is read once per process, so everything lives
//! in one test function.
#![cfg(feature = "enabled")]

#[lind_perf::timed]
fn plain() -> u32 {
    std::hint::black_box(1)
}

#[lind_perf::timed(key = n)]
fn keyed(n: u64) -> u64 {
    std::hint::black_box(n)
}

#[lind_perf::timed(name = "custom::skipped")]
fn skipped() {}

#[lind_perf::timed(name = "custom::method_near_miss")]
fn near_miss() {}

struct S;
impl S {
    #[lind_perf::timed(name = "custom::method")]
    fn method(&self, x: u32) -> u32 {
        if x == 0 {
            return 7; // early return still records
        }
        x
    }
}

#[test]
fn report_contents() {
    let out = std::env::temp_dir().join(format!("lind-perf-test-{}.tsv", std::process::id()));
    let _ = std::fs::remove_file(&out);
    std::env::set_var("LIND_PERF", "clock");
    std::env::set_var("LIND_PERF_ONLY", "enabled::,custom::method");
    std::env::set_var("LIND_PERF_OUT", &out);
    lind_perf::init();

    for _ in 0..10 {
        plain();
    }
    for k in [3u64, 3, 107, 5000] {
        keyed(k);
    }
    skipped();
    near_miss();
    assert_eq!(S.method(0), 7);
    assert_eq!(S.method(2), 2);
    {
        let _g = lind_perf::scope!("enabled::block");
    }

    lind_perf::report();
    lind_perf::report(); // second call is a no-op

    let text = std::fs::read_to_string(&out).unwrap();
    let _ = std::fs::remove_file(&out);
    print!("{text}");

    let row = |name: &str, key: &str| -> Option<u64> {
        text.lines()
            .map(|l| l.split('\t').collect::<Vec<_>>())
            .find(|c| c.len() == 5 && c[0] == name && c[1] == key)
            .map(|c| c[2].parse().unwrap())
    };

    assert_eq!(text.matches("# lind-perf").count(), 1);
    assert_eq!(row("enabled::plain", "-"), Some(10));
    assert_eq!(row("enabled::keyed", "3"), Some(2));
    assert_eq!(row("enabled::keyed", "107"), Some(1));
    assert_eq!(row("enabled::keyed", "other"), Some(1));
    assert_eq!(row("custom::method", "-"), Some(2));
    assert_eq!(row("enabled::block", "-"), Some(1));
    // Ran, but filtered out by LIND_PERF_ONLY: listed with zero calls so drivers can discover it.
    assert_eq!(row("custom::skipped", "-"), Some(0));
    // Exact-name entries do not act as prefixes.
    assert_eq!(row("custom::method_near_miss", "-"), Some(0));

    // Numeric key order: 3 before 107.
    let i3 = text.find("enabled::keyed\t3\t").unwrap();
    let i107 = text.find("enabled::keyed\t107\t").unwrap();
    assert!(i3 < i107);
}
