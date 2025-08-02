#!/bin/bash

/home/lind/lind-wasm/clang+llvm-16.0.4-x86_64-linux-gnu-ubuntu-22.04/bin/clang -pthread --target=wasm32-unknown-wasi --sysroot /home/lind/lind-wasm/src/glibc/sysroot -DEUID_GRATE_VAL=$EUID_GRATE_VAL -Wl,--import-memory,--export-memory,--max-memory=67108864,--export=__stack_pointer,--export=__stack_low,--export=pass_fptr_to_wt,--export-table open_grate.c -g -O0 -o open_grate.wasm && wasm-opt --asyncify --debuginfo open_grate.wasm -o open_grate.wasm && /home/lind/lind-wasm/src/wasmtime/target/debug/wasmtime compile open_grate.wasm -o open_grate.cwasm
