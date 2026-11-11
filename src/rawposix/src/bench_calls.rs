//! Benchmarking syscalls (2001-2999).
//!
//! These exist only to measure the cost of the syscall path; lind-perf gates its syscall-path
//! timers on this number range. Exposed to guests through glibc's `lind_syscall.h`.

use crate::fs_calls::close_syscall;
use crate::sys_calls::geteuid_syscall;
use typemap::datatype_conversion::sc_unusedarg;

/// BM_GRATE (2001): does no work. Measures bare 3i -> RawPOSIX dispatch; a grate can register
/// a handler for this number to measure the grate round trip instead.
pub extern "C" fn bm_grate_syscall(
    _cageid: u64,
    arg1: u64,
    arg1_cageid: u64,
    arg2: u64,
    arg2_cageid: u64,
    arg3: u64,
    arg3_cageid: u64,
    arg4: u64,
    arg4_cageid: u64,
    arg5: u64,
    arg5_cageid: u64,
    arg6: u64,
    arg6_cageid: u64,
) -> i32 {
    0
}

/// BM_RAWPOSIX (2002): a syscall resolved through the host kernel. Behaves like geteuid().
#[lind_perf::timed]
pub extern "C" fn bm_rawposix_syscall(
    cageid: u64,
    arg1: u64,
    arg1_cageid: u64,
    arg2: u64,
    arg2_cageid: u64,
    arg3: u64,
    arg3_cageid: u64,
    arg4: u64,
    arg4_cageid: u64,
    arg5: u64,
    arg5_cageid: u64,
    arg6: u64,
    arg6_cageid: u64,
) -> i32 {
    geteuid_syscall(
        cageid,
        arg1,
        arg1_cageid,
        arg2,
        arg2_cageid,
        arg3,
        arg3_cageid,
        arg4,
        arg4_cageid,
        arg5,
        arg5_cageid,
        arg6,
        arg6_cageid,
    )
}

/// BM_FDTABLES (2003): a syscall resolved in fdtables without the kernel. Behaves like
/// close(-1); glibc hardcodes the fd to -1 so it always fails with EBADF.
#[lind_perf::timed]
pub extern "C" fn bm_fdtables_syscall(
    cageid: u64,
    vfd_arg: u64,
    vfd_cageid: u64,
    arg2: u64,
    arg2_cageid: u64,
    arg3: u64,
    arg3_cageid: u64,
    arg4: u64,
    arg4_cageid: u64,
    arg5: u64,
    arg5_cageid: u64,
    arg6: u64,
    arg6_cageid: u64,
) -> i32 {
    close_syscall(
        cageid,
        vfd_arg,
        vfd_cageid,
        arg2,
        arg2_cageid,
        arg3,
        arg3_cageid,
        arg4,
        arg4_cageid,
        arg5,
        arg5_cageid,
        arg6,
        arg6_cageid,
    )
}
