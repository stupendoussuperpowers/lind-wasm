use crate::timers::{read_end, read_start, TimerKind};
use std::io::Write;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicU8, Ordering::*};
use std::sync::OnceLock;

/// Buckets for a keyed counter. Keys at or past the last index all go into the last bucket,
/// which is reported as key `other`. 2048 covers Linux syscall numbers plus lind's 1000-range
/// 3i calls.
pub const KEYED_BUCKETS: usize = 2048;

const UNINIT: u8 = 0;
const OFF: u8 = 1;
const ON: u8 = 2;
const BUSY: u8 = 3;

/// Totals for one (counter, key).
pub struct Bucket {
    calls: AtomicU64,
    total: AtomicU64,
}

impl Bucket {
    pub const fn new() -> Self {
        Self {
            calls: AtomicU64::new(0),
            total: AtomicU64::new(0),
        }
    }
}

/// One callsite. Created by `scope!` as a hidden static; never constructed by hand.
pub struct Counter {
    name: &'static str,
    keyed: bool,
    buckets: &'static [Bucket],
    state: AtomicU8,
    /// Intrusive link in the global list of counters that have run at least once.
    next: AtomicPtr<Counter>,
}

/// Head of the list of every counter that has executed. Push-only, so no locking.
static HEAD: AtomicPtr<Counter> = AtomicPtr::new(ptr::null_mut());
/// Timer kind for every counter, fixed at config init.
static TIMER: AtomicU8 = AtomicU8::new(TimerKind::Clock as u8);
static REPORTED: AtomicBool = AtomicBool::new(false);

impl Counter {
    pub const fn new(name: &'static str, keyed: bool, buckets: &'static [Bucket]) -> Self {
        Self {
            name,
            keyed,
            buckets,
            state: AtomicU8::new(UNINIT),
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }

    #[inline(always)]
    pub fn guard(&'static self, key: u64) -> Guard {
        let mut s = self.state.load(Relaxed);
        if s == UNINIT {
            s = self.register();
        }
        if s == ON {
            let kind = TimerKind::from_u8(TIMER.load(Relaxed));
            Guard {
                counter: Some(self),
                key,
                kind,
                start: read_start(kind),
            }
        } else {
            Guard {
                counter: None,
                key: 0,
                kind: TimerKind::Clock,
                start: 0,
            }
        }
    }

    /// First execution of this callsite: decide on/off and link into the global list.
    /// Threads that lose the race skip timing for this one call.
    #[cold]
    #[inline(never)]
    fn register(&'static self) -> u8 {
        if self
            .state
            .compare_exchange(UNINIT, BUSY, Acquire, Relaxed)
            .is_err()
        {
            return OFF;
        }
        let cfg = config();
        let me = self as *const Counter as *mut Counter;
        let mut head = HEAD.load(Relaxed);
        loop {
            self.next.store(head, Relaxed);
            match HEAD.compare_exchange_weak(head, me, Release, Relaxed) {
                Ok(_) => break,
                Err(h) => head = h,
            }
        }
        let s = if cfg.wants(self.name) { ON } else { OFF };
        self.state.store(s, Release);
        s
    }

    #[inline(always)]
    fn record(&self, key: u64, elapsed: u64) {
        let idx = if self.keyed {
            (key as usize).min(self.buckets.len() - 1)
        } else {
            0
        };
        let b = &self.buckets[idx];
        b.calls.fetch_add(1, Relaxed);
        b.total.fetch_add(elapsed, Relaxed);
    }
}

/// RAII timer returned by `scope!`. Records on drop.
pub struct Guard {
    counter: Option<&'static Counter>,
    key: u64,
    kind: TimerKind,
    start: u64,
}

impl Drop for Guard {
    #[inline(always)]
    fn drop(&mut self) {
        if let Some(c) = self.counter {
            let end = read_end(self.kind);
            c.record(self.key, end.saturating_sub(self.start));
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Config (environment, read once)
// ---------------------------------------------------------------------------------------------

struct Config {
    active: bool,
    kind: TimerKind,
    only: Vec<String>,
    /// Opened at config init, so a path given before lind-boot chroots still points at the host
    /// filesystem as long as [`init`] runs first.
    out: Option<(String, std::io::Result<std::fs::File>)>,
}

impl Config {
    fn from_env() -> Self {
        let mode = std::env::var("LIND_PERF").unwrap_or_default();
        let (active, kind) = match mode.trim().to_ascii_lowercase().as_str() {
            "" | "0" | "off" | "false" => (false, TimerKind::Clock),
            "tsc" | "rdtsc" => (true, TimerKind::Rdtsc),
            _ => (true, TimerKind::Clock),
        };
        let only = std::env::var("LIND_PERF_ONLY")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let out = std::env::var("LIND_PERF_OUT")
            .ok()
            .filter(|s| !s.is_empty() && active)
            .map(|path| {
                let f = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path);
                (path, f)
            });
        Self {
            active,
            kind,
            only,
            out,
        }
    }

    /// `LIND_PERF_ONLY` entries match a counter name exactly, or as a prefix when they end in
    /// `::` (module) or `*` (anything).
    fn wants(&self, name: &str) -> bool {
        self.active
            && (self.only.is_empty()
                || self.only.iter().any(|p| {
                    if let Some(pre) = p.strip_suffix('*') {
                        name.starts_with(pre)
                    } else if p.ends_with("::") {
                        name.starts_with(p.as_str())
                    } else {
                        name == p
                    }
                }))
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

extern "C" fn report_at_exit() {
    report();
}

fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let c = Config::from_env();
        TIMER.store(c.kind as u8, Relaxed);
        if c.active {
            // Covers std::process::exit, which lind-boot uses on every exit path.
            unsafe { libc::atexit(report_at_exit) };
        }
        c
    })
}

/// Read the environment and open `LIND_PERF_OUT` now instead of on the first timed call.
///
/// Optional. Call it before anything changes the process root or cwd (lind-boot chroots into
/// lindfs), so a relative or host path in `LIND_PERF_OUT` resolves where the caller expects.
pub fn init() {
    let _ = config();
}

// ---------------------------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------------------------

fn counters() -> impl Iterator<Item = &'static Counter> {
    let mut cur = HEAD.load(Acquire);
    std::iter::from_fn(move || {
        if cur.is_null() {
            return None;
        }
        // SAFETY: only `&'static Counter`s are ever pushed, and nothing is removed.
        let c: &'static Counter = unsafe { &*cur };
        cur = c.next.load(Relaxed);
        Some(c)
    })
}

/// Mean cost of an empty guard (start read + end read + record), in report units. Lets readers
/// correct small or nested numbers.
fn calibrate(kind: TimerKind) -> u64 {
    const N: u64 = 20_000;
    let sink = Bucket::new();
    let mut best = u64::MAX;
    for _ in 0..5 {
        let t0 = read_start(kind);
        for _ in 0..N {
            let s = read_start(kind);
            let e = read_end(kind);
            sink.calls.fetch_add(1, Relaxed);
            sink.total.fetch_add(e.saturating_sub(s), Relaxed);
        }
        let t1 = read_end(kind);
        best = best.min(t1.saturating_sub(t0) / N);
    }
    best
}

/// Write the report now. Called automatically at exit; calling it yourself suppresses the
/// automatic one.
///
/// Format (tab-separated, `#` lines are metadata):
///
/// ```text
/// # lind-perf timer=clock unit=ns overhead=31 pid=4242 only=threei::
/// name	key	calls	total	avg
/// threei::threei::make_syscall	107	1000012	412300000	412
/// ```
///
/// `key` is `-` for unkeyed counters. Every counter that ran is listed, including disabled ones
/// (with zero calls), so a driver can discover which counters exist.
pub fn report() {
    let Some(cfg) = CONFIG.get() else { return };
    if !cfg.active || REPORTED.swap(true, AcqRel) {
        return;
    }

    let mut buf = Vec::<u8>::new();
    let _ = writeln!(
        buf,
        "# lind-perf timer={} unit={} overhead={} pid={} only={}",
        match cfg.kind {
            TimerKind::Clock => "clock",
            TimerKind::Rdtsc => "tsc",
        },
        cfg.kind.unit(),
        calibrate(cfg.kind),
        std::process::id(),
        if cfg.only.is_empty() {
            "*".to_string()
        } else {
            cfg.only.join(",")
        },
    );
    let _ = writeln!(buf, "name\tkey\tcalls\ttotal\tavg");

    // (name, sort key, label, calls, total)
    let mut rows: Vec<(&str, usize, String, u64, u64)> = Vec::new();
    for c in counters() {
        let mut any = false;
        for (i, b) in c.buckets.iter().enumerate() {
            let calls = b.calls.load(Relaxed);
            if calls == 0 {
                continue;
            }
            any = true;
            let key = if !c.keyed {
                "-".to_string()
            } else if i == c.buckets.len() - 1 {
                "other".to_string()
            } else {
                i.to_string()
            };
            rows.push((c.name, i, key, calls, b.total.load(Relaxed)));
        }
        if !any {
            rows.push((c.name, 0, "-".to_string(), 0, 0));
        }
    }
    rows.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(&b.1)));
    for (name, _, key, calls, total) in rows {
        let avg = if calls == 0 { 0 } else { total / calls };
        let _ = writeln!(buf, "{name}\t{key}\t{calls}\t{total}\t{avg}");
    }

    match &cfg.out {
        Some((path, file)) => {
            let r = match file {
                Ok(f) => {
                    let mut w: &std::fs::File = f;
                    w.write_all(&buf)
                }
                Err(e) => Err(std::io::Error::new(e.kind(), e.to_string())),
            };
            if let Err(e) = r {
                eprintln!("lind-perf: cannot write {path}: {e}");
                let _ = std::io::stderr().write_all(&buf);
            }
        }
        None => {
            let _ = std::io::stderr().write_all(&buf);
        }
    }
}

/// Zero every counter (for example between phases of an in-process benchmark).
pub fn reset() {
    for c in counters() {
        for b in c.buckets {
            b.calls.store(0, Relaxed);
            b.total.store(0, Relaxed);
        }
    }
    REPORTED.store(false, Release);
}
