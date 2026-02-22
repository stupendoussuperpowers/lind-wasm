use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::time::Instant;

pub struct Counter {
    pub cycles: AtomicU64,
    pub calls: AtomicU64,
    pub name: &'static str,
    pub enabled: AtomicBool,
}

impl Counter {
    pub const fn new(name: &'static str) -> Self {
        Self {
            cycles: AtomicU64::new(0),
            calls: AtomicU64::new(0),
            name,
            enabled: AtomicBool::new(false),
        }
    }

    #[inline(always)]
    pub fn start(&self) -> Sample {
        if self.enabled.load(Ordering::Relaxed) {
            let source = current_timer_source();
            Sample {
                start: read_start(source),
                source: source.as_u8(),
            }
        } else {
            Sample {
                start: 0,
                source: TimerSource::disabled_tag(),
            }
        }
    }

    #[inline(always)]
    pub fn record(&self, sample: Sample) {
        if self.enabled.load(Ordering::Relaxed) {
            if sample.source == TimerSource::disabled_tag() {
                return;
            }
            let source = TimerSource::from_u8(sample.source).unwrap_or_else(current_timer_source);
            let elapsed = read_end(source).saturating_sub(sample.start);
            self.cycles.fetch_add(elapsed, Ordering::Relaxed);
            self.calls.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub fn scope(&self) -> Scope<'_> {
        Scope {
            counter: self,
            sample: self.start(),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.cycles.store(0, Ordering::Relaxed);
        self.calls.store(0, Ordering::Relaxed);
    }
}

pub struct Scope<'a> {
    counter: &'a Counter,
    sample: Sample,
}

impl Drop for Scope<'_> {
    fn drop(&mut self) {
        self.counter.record(self.sample);
    }
}

#[inline(always)]
fn read_start(source: TimerSource) -> u64 {
    match source {
        TimerSource::Rdtsc => rdtsc_start(),
        TimerSource::ClockGettime => clock_now_nanos(),
    }
}

#[inline(always)]
fn read_end(source: TimerSource) -> u64 {
    match source {
        TimerSource::Rdtsc => rdtsc_end(),
        TimerSource::ClockGettime => clock_now_nanos(),
    }
}

#[inline(always)]
fn rdtsc_start() -> u64 {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_lfence();
        #[cfg(target_arch = "x86")]
        core::arch::x86::_mm_lfence();
        #[cfg(target_arch = "x86_64")]
        return core::arch::x86_64::_rdtsc();
        #[cfg(target_arch = "x86")]
        return core::arch::x86::_rdtsc();
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    {
        clock_now_nanos()
    }
}

#[inline(always)]
fn rdtsc_end() -> u64 {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe {
        let mut aux = 0u32;
        #[cfg(target_arch = "x86_64")]
        let tsc = core::arch::x86_64::__rdtscp(&mut aux);
        #[cfg(target_arch = "x86")]
        let tsc = core::arch::x86::__rdtscp(&mut aux);
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_lfence();
        #[cfg(target_arch = "x86")]
        core::arch::x86::_mm_lfence();
        tsc
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    {
        clock_now_nanos()
    }
}

#[inline(always)]
fn instant_nanos() -> u64 {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START.get_or_init(Instant::now);
    start.elapsed().as_nanos() as u64
}

#[inline(always)]
fn clock_now_nanos() -> u64 {
    #[cfg(unix)]
    {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let rc = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, &mut ts) };
        if rc == 0 {
            return (ts.tv_sec as u64)
                .saturating_mul(1_000_000_000)
                .saturating_add(ts.tv_nsec as u64);
        }
        return instant_nanos();
    }
    #[cfg(not(unix))]
    {
        instant_nanos()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimerSource {
    Rdtsc,
    ClockGettime,
}

impl TimerSource {
    const fn as_u8(self) -> u8 {
        match self {
            TimerSource::Rdtsc => 0,
            TimerSource::ClockGettime => 1,
        }
    }

    const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(TimerSource::Rdtsc),
            1 => Some(TimerSource::ClockGettime),
            _ => None,
        }
    }

    const fn disabled_tag() -> u8 {
        u8::MAX
    }

    pub fn unit(self) -> &'static str {
        match self {
            TimerSource::Rdtsc => "cycles",
            TimerSource::ClockGettime => "ns",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sample {
    start: u64,
    source: u8,
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const DEFAULT_SOURCE: TimerSource = TimerSource::Rdtsc;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
const DEFAULT_SOURCE: TimerSource = TimerSource::ClockGettime;

static TIMER_SOURCE: AtomicU8 = AtomicU8::new(DEFAULT_SOURCE.as_u8());

pub fn set_timer_source(source: TimerSource) {
    TIMER_SOURCE.store(source.as_u8(), Ordering::Relaxed);
}

pub fn current_timer_source() -> TimerSource {
    TimerSource::from_u8(TIMER_SOURCE.load(Ordering::Relaxed)).unwrap_or(DEFAULT_SOURCE)
}

pub fn set_timer_source_from_env(var_name: &str) -> Option<TimerSource> {
    let value = std::env::var(var_name).ok()?;
    let lower = value.trim().to_ascii_lowercase();
    let source = match lower.as_str() {
        "rdtsc" | "tsc" | "cycles" | "0" => TimerSource::Rdtsc,
        "clock" | "clock_gettime" | "clockgettime" | "monotonic" | "ns" | "1" => {
            TimerSource::ClockGettime
        }
        _ => return None,
    };
    set_timer_source(source);
    Some(source)
}

pub fn enable_all(counters: &[&Counter]) {
    for c in counters {
        c.enable();
    }
}

pub fn disable_all(counters: &[&Counter]) {
    for c in counters {
        c.disable();
    }
}

pub fn reset_all(counters: &[&Counter]) {
    for c in counters {
        c.reset();
    }
}

pub fn report(counters: &[&Counter]) {
    let source = current_timer_source();
    let unit = source.unit();
    eprintln!("\n=== lind-perf report ===");
    eprintln!("timer_source={source:?}, unit={unit}");
    for c in counters {
        let calls = c.calls.load(Ordering::Relaxed);
        if calls == 0 {
            continue;
        }
        let cycles = c.cycles.load(Ordering::Relaxed);
        eprintln!(
            "{}: calls={}, total_{}={}, avg_{}={}",
            c.name,
            calls,
            unit,
            cycles,
            unit,
            cycles / calls
        );
    }
}

#[macro_export]
macro_rules! scope {
    ($counter:expr) => {
        let _lind_perf_scope = $counter.scope();
    };
}
