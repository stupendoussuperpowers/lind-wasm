#!/usr/bin/env python3
"""Break benchmark time down by lind-perf counter.

Runs tests/benchmarks programs (same names/patterns as benchrunner.py) and reports where the time
goes inside the runtime. build/lind-boot must be built with lind-perf:

  make lind-boot NO_LOGGING=1 LIND_BOOT_EXTRA_FEATURES=lind_perf

For each benchmark:
  1. discovery run with every counter on, to learn which counters the workload hits;
  2. one run per counter with only that counter on (unless --all-at-once), so timer overhead
     from one counter never leaks into another's numbers;
  3. each run repeated --repeat times; the median per-call average is reported.

Examples:
  ./scripts/test/perfrunner.py sys_geteuid sys_close
  ./scripts/test/perfrunner.py --timer tsc --repeat 5 -o perf.csv sys_
  ./scripts/test/perfrunner.py geteuid-grate.sys_geteuid.grate
  ./scripts/test/perfrunner.py --cmd -- scripts/bin/lind_run some/other.cwasm
"""
import argparse
import csv
import os
import statistics
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import benchrunner as br  # noqa: E402

# x86_64 numbers used by lind (see src/glibc/lind_syscall/lind_syscall_num.h).
SYSCALL_NAMES = {
    0: "read", 1: "write", 2: "open", 3: "close", 8: "lseek", 9: "mmap", 11: "munmap",
    12: "brk", 17: "pread64", 18: "pwrite64", 22: "pipe", 32: "dup", 39: "getpid",
    41: "socket", 42: "connect", 43: "accept", 44: "sendto", 45: "recvfrom", 49: "bind",
    50: "listen", 56: "clone", 57: "fork", 59: "execve", 60: "exit", 61: "wait4",
    102: "getuid", 104: "getgid", 107: "geteuid", 108: "getegid", 202: "futex",
    228: "clock_gettime", 231: "exit_group", 293: "pipe2",
}


def key_label(key: str) -> str:
    if key.isdigit() and int(key) in SYSCALL_NAMES:
        return f"{key}:{SYSCALL_NAMES[int(key)]}"
    return key


def parse_report(text: str):
    """Return (meta, rows). rows: {(name, key): (calls, total, avg)}."""
    meta, rows = {}, {}
    for line in text.splitlines():
        if line.startswith("# lind-perf"):
            # A process may append more than one report (e.g. a forked host); keep the last
            # header but sum rows.
            meta = dict(kv.split("=", 1) for kv in line.split()[2:] if "=" in kv)
            continue
        parts = line.split("\t")
        if len(parts) != 5 or parts[0] == "name":
            continue
        name, key, calls, total, _ = parts
        c0, t0, _ = rows.get((name, key), (0, 0, 0))
        calls, total = c0 + int(calls), t0 + int(total)
        rows[(name, key)] = (calls, total, total // calls if calls else 0)
    return meta, rows


def run_once(cmd, timer, only):
    """Run cmd once with lind-perf configured. Returns (meta, rows, bench_results)."""
    fd, out = tempfile.mkstemp(prefix="lind-perf-", suffix=".tsv")
    os.close(fd)
    env = dict(os.environ)
    env["LIND_PERF"] = timer
    env["LIND_PERF_OUT"] = out
    if only:
        env["LIND_PERF_ONLY"] = only
    else:
        env.pop("LIND_PERF_ONLY", None)
    try:
        status = br.subprocess.run(cmd, env=env, timeout=600,
                                   stdout=br.subprocess.PIPE, stderr=br.subprocess.PIPE)
        bench = {}
        br.parse_output(bench, status.stdout, "lind")
        if status.returncode != 0:
            print(f"  warning: exit {status.returncode}: {status.stderr.decode(errors='replace')[-500:]}")
        with open(out) as f:
            meta, rows = parse_report(f.read())
        return meta, rows, bench
    finally:
        try:
            os.unlink(out)
        except OSError:
            pass


def profile(label, cmd, args):
    print(f"Profiling: {label}")
    meta, rows, bench = run_once(cmd, args.timer, args.only)
    if not rows:
        print("  no lind-perf report; was build/lind-boot built with `make lind-boot NO_LOGGING=1 LIND_BOOT_EXTRA_FEATURES=lind_perf`?")
        return []
    names = sorted({n for (n, _) in rows})

    # (name, key) -> list of per-call averages across repeats
    samples, calls_seen = {}, {}
    plan = [(None, args.only)] if args.all_at_once else [(n, n) for n in names]
    for name, only in plan:
        for _ in range(args.repeat):
            m, r, _ = run_once(cmd, args.timer, only)
            meta = m or meta
            for (n, k), (calls, _, avg) in r.items():
                if calls == 0 or (name is not None and n != name):
                    continue
                samples.setdefault((n, k), []).append(avg)
                calls_seen[(n, k)] = calls

    # End-to-end numbers the benchmark itself printed (from the discovery run).
    e2e = "; ".join(f"{t}[{p}]={v['lind']}ns" for t, ps in bench.items() for p, v in ps.items())

    out = []
    for (n, k) in sorted(samples, key=lambda nk: (nk[0], int(nk[1]) if nk[1].isdigit() else 1 << 30)):
        if args.min_calls and calls_seen[(n, k)] < args.min_calls:
            continue
        out.append({
            "bench": label, "counter": n, "key": key_label(k), "calls": calls_seen[(n, k)],
            "avg": int(statistics.median(samples[(n, k)])), "unit": meta.get("unit", "?"),
            "overhead": meta.get("overhead", "?"), "e2e": e2e,
        })
    return out


def print_table(rows):
    if not rows:
        return
    cols = ["bench", "counter", "key", "calls", "avg", "unit", "overhead"]
    w = {c: max(len(c), *(len(str(r[c])) for r in rows)) for c in cols}
    print("  ".join(c.upper().ljust(w[c]) for c in cols))
    print("  ".join("-" * w[c] for c in cols))
    last = None
    for r in rows:
        if r["bench"] != last and last is not None:
            print()
        last = r["bench"]
        print("  ".join(str(r[c]).ljust(w[c]) for c in cols))
    print()
    for b in dict.fromkeys(r["bench"] for r in rows):
        e2e = next(r["e2e"] for r in rows if r["bench"] == b)
        if e2e:
            print(f"{b} end-to-end (benchmark's own timer): {e2e}")


def bench_commands(patterns):
    for test in br.collect_tests(patterns):
        if test.suffix == ".c":
            wasm = br.compile_lind(test)
            if not wasm:
                print(f"Failed to compile {test}. Skipping.")
                continue
            yield test.stem, [str(br.LIND_TOOL_PATH / "lind_run"), wasm]
        elif test.suffix == ".grate":
            bins = []
            for part in test.with_suffix("").name.split("."):
                if part.endswith("-grate"):
                    d = br.resolve_grate_dir(part)
                    b = d and br.compile_grate(d)
                else:
                    b = br.compile_lind(br.BENCH_DIR / f"{part}.c")
                if not b:
                    bins = None
                    break
                bins.append(b)
            if not bins:
                print(f"Failed to compile {test}. Skipping.")
                continue
            yield test.stem, [str(br.LIND_TOOL_PATH / "lind_run")] + bins


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("patterns", nargs="*", help="benchmark filename prefixes (as in benchrunner.py)")
    p.add_argument("--timer", choices=["clock", "tsc"], default="clock")
    p.add_argument("--repeat", type=int, default=3, help="runs per counter (median reported)")
    p.add_argument("--all-at-once", action="store_true",
                   help="single run with every counter on (faster, nested numbers inflated)")
    p.add_argument("--only", default="", help="restrict to these LIND_PERF_ONLY entries")
    p.add_argument("--min-calls", type=int, default=1000,
                   help="hide (counter, key) rows with fewer calls, e.g. startup syscalls")
    p.add_argument("-o", "--out", help="write CSV here instead of a table")
    p.add_argument("--cmd", nargs=argparse.REMAINDER,
                   help="profile an arbitrary command instead of tests/benchmarks")
    args = p.parse_args()

    rows = []
    if args.cmd:
        cmd = args.cmd[1:] if args.cmd and args.cmd[0] == "--" else args.cmd
        rows += profile(" ".join(Path(c).name for c in cmd), cmd, args)
    else:
        for label, cmd in bench_commands(args.patterns):
            rows += profile(label, cmd, args)

    if args.out:
        with open(args.out, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0].keys()) if rows else ["bench"])
            w.writeheader()
            w.writerows(rows)
        print(f"Wrote {args.out}")
    else:
        print_table(rows)


if __name__ == "__main__":
    main()
