#!/bin/bash

./scripts/lind_run src/RawPOSIX/tmp/geteuid_grate.wasm middle_grate.wasm middle_grate.wasm middle_grate.wasm benchmark.wasm > 4grate-ctx

./stat.py 4grate-ctx

head -2 4grate-ctx_out.csv
