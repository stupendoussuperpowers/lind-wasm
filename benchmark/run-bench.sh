#!/bin/bash

./scripts/lind_run src/tmp/geteuid_grate.wasm middle_grate.wasm benchmark.wasm > "$@"

head -20 "$@"
