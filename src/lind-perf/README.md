# lind-perf

Scope timers for lind's hot paths. Compiled out unless the `enabled` feature is on.

```rust
#[lind_perf::timed]                     // one counter
#[lind_perf::timed(key = syscall_num)]  // one row per key
let g = lind_perf::scope!("name");      // a block; ends on drop
```

Crates only need `lind-perf = { path = "../lind-perf" }`. lind-boot's `lind_perf` feature turns
it on everywhere.

## Running

```sh
make lind-boot NO_LOGGING=1 LIND_BOOT_EXTRA_FEATURES=lind_perf
./scripts/test/perfrunner.py sys_geteuid sys_close
```

Or set these by hand when running lind-boot:

- `LIND_PERF`: `clock` (ns) or `tsc` (cycles). Unset means off.
- `LIND_PERF_ONLY`: comma list of names, or prefixes ending in `::` or `*`.
- `LIND_PERF_OUT`: file to append the TSV report to. Default: stderr.
