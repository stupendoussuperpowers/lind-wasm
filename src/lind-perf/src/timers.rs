/// Timer backend.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerKind {
    /// `clock_gettime(CLOCK_MONOTONIC_RAW)`, reported in nanoseconds.
    Clock = 0,
    /// `rdtsc` / `rdtscp` with lfences, reported in TSC ticks. Falls back to `Clock` off x86_64.
    Rdtsc = 1,
}

impl TimerKind {
    pub fn unit(self) -> &'static str {
        match self {
            TimerKind::Clock => "ns",
            TimerKind::Rdtsc => "cycles",
        }
    }

    pub(crate) fn from_u8(v: u8) -> Self {
        match v {
            1 => TimerKind::Rdtsc,
            _ => TimerKind::Clock,
        }
    }
}

#[inline(always)]
pub(crate) fn read_start(kind: TimerKind) -> u64 {
    match kind {
        TimerKind::Rdtsc => rdtsc_start(),
        TimerKind::Clock => clock_now(),
    }
}

#[inline(always)]
pub(crate) fn read_end(kind: TimerKind) -> u64 {
    match kind {
        TimerKind::Rdtsc => rdtsc_end(),
        TimerKind::Clock => clock_now(),
    }
}

#[inline(always)]
fn rdtsc_start() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // Keep earlier instructions from drifting past the timestamp read.
        core::arch::x86_64::_mm_lfence();
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    clock_now()
}

#[inline(always)]
fn rdtsc_end() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // RDTSCP waits for prior instructions; the fence keeps later ones from starting early.
        let mut aux = 0u32;
        let t = core::arch::x86_64::__rdtscp(&mut aux);
        core::arch::x86_64::_mm_lfence();
        t
    }
    #[cfg(not(target_arch = "x86_64"))]
    clock_now()
}

#[inline(always)]
fn clock_now() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // CLOCK_MONOTONIC_RAW cannot fail on Linux/macOS with a valid pointer; ignore rc so the hot
    // path does not carry a panic branch.
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC_RAW, &mut ts) };
    (ts.tv_sec as u64)
        .wrapping_mul(1_000_000_000)
        .wrapping_add(ts.tv_nsec as u64)
}
