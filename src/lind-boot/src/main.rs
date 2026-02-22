mod cli;
mod lind_wasmtime;
mod perf;

use crate::{
    cli::CliOptions,
    lind_wasmtime::{execute_wasmtime, precompile_module},
};
use clap::Parser;
use rawposix::init::{rawposix_shutdown, rawposix_start};
use std::ffi::OsString;

/// Entry point of the lind-boot executable.
///
/// The expected invocation follows: the first non-flag argument specifies the
/// Wasm binary to execute and all remaining arguments are forwarded verbatim to
/// the guest program:
///
///     lind-boot [flags...] wasm_file.wasm arg1 arg2 ...
///
/// All process lifecycle management, runtime initialization, and error
/// handling semantics are delegated to `execute.rs`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lindboot_cli = CliOptions::parse();

    #[cfg(feature = "lind_perf")]
    {
        if lindboot_cli.perf_sweep {
            if lindboot_cli.precompile {
                return Err("--perf-sweep cannot be used with --precompile".into());
            }
            if !lindboot_cli.perf {
                return Err("--perf-sweep requires --perf".into());
            }
            let exe = std::env::current_exe()?;
            let names = perf::enabled::all_counter_names();
            for name in names {
                let mut cmd = std::process::Command::new(&exe);
                for _ in 0..lindboot_cli.verbose {
                    cmd.arg("-v");
                }
                if lindboot_cli.debug {
                    cmd.arg("--debug");
                }
                if lindboot_cli.backtrace {
                    cmd.arg("--backtrace");
                }
                for (k, v) in &lindboot_cli.vars {
                    cmd.arg("--env");
                    match v {
                        Some(val) => cmd.arg(format!("{k}={val}")),
                        None => cmd.arg(OsString::from(k)),
                    };
                }

                cmd.arg("--perf");
                cmd.arg("--perf-report");
                cmd.arg("--perf-only");
                cmd.arg(name);
                if let Some(source) = lindboot_cli.perf_source.as_deref() {
                    cmd.arg("--perf-source");
                    cmd.arg(source);
                }
                // preserve the env-based selector too if user used it
                cmd.envs(std::env::vars());

                // guest argv (wasm path + args)
                cmd.args(&lindboot_cli.args);

                let status = cmd.status()?;
                if !status.success() {
                    return Err(format!("perf sweep failed for counter `{name}`: {status}").into());
                }
            }
            return Ok(());
        }

        if let Some(source) = lindboot_cli.perf_source.as_deref() {
            if !perf::enabled::set_timer_source(source) {
                return Err(format!(
                    "invalid --perf-source `{source}` (expected `rdtsc` or `clock_gettime`)"
                )
                .into());
            }
        }
        if lindboot_cli.perf {
            perf::enabled::reset_all();
            if let Some(name) = lindboot_cli.perf_only.as_deref() {
                if !perf::enabled::enable_only(name) {
                    return Err(format!("unknown --perf-only counter name `{name}`").into());
                }
            } else {
                perf::enabled::enable_all();
            }
        } else {
            perf::enabled::disable_all();
        }
    }

    // AOT-compile only — no runtime needed
    if lindboot_cli.precompile {
        precompile_module(&lindboot_cli)?;
        return Ok(());
    }

    // Initialize RawPOSIX and register RawPOSIX syscalls with 3i
    rawposix_start(0);

    // Execute with user-selected runtime. Can be switched to other runtime implementation
    // in the future (e.g.: MPK).
    let run_result = execute_wasmtime(lindboot_cli.clone());

    // after all cage exits, finalize the lind
    rawposix_shutdown();

    #[cfg(feature = "lind_perf")]
    {
        if lindboot_cli.perf_report {
            perf::enabled::report();
        }
    }

    run_result?;

    Ok(())
}
