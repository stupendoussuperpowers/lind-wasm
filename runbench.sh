#!/bin/bash

for file in benchmarks/*.c; do
    echo "================"
    echo "Benchmark: $file"

    cc "$file" -o "${file%.c}.o" &&
    echo -n "Linux:" && "./${file%.c}.o"

    ./scripts/lind_compile "$file" &>/dev/null  &&
    echo -n "Lind (RawPOSIX):" && ./scripts/lind_run "${file%.c}.wasm"
done
