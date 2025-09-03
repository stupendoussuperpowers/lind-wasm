#!/bin/bash

set -x 

cd /home/lind/lind-wasm/

src/wasmtime/target/release/wasmtime run --env PRELOADS="$(paste -sd: imfs_grate/preloads)" --allow-precompiled --wasi threads=y --wasi preview2=n imfs_grate/open_grate.wasm tcc.wasm
