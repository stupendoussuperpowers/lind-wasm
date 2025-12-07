#!/usr/bin/env python3
import sys
import statistics
from collections import defaultdict
import re

UNIT_MULT = {
    "ns": 1,
    "µs": 1000,
    "us": 1000,     # allow ASCII micro as well
    "ms": 1_000_000,
    "s":  1_000_000_000,
}

def parse_time(text):
    """
    Convert '4.162µs', '622ns', '1.2ms', etc. to nanoseconds (float).
    """
    text = text.strip()
    match = re.match(r"([\d\.]+)\s*([a-zA-Zµ]+)", text)
    if not match:
        raise ValueError(f"Unrecognized time format: {text}")
    value, unit = match.groups()
    if unit not in UNIT_MULT:
        raise ValueError(f"Unknown unit: {unit}")
    return float(value) * UNIT_MULT[unit]

def main(path):
    groups = defaultdict(list)

    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or ":" not in line:
                continue

            label_part, rest = line.split(":", 1)
            label = label_part.strip()

            # format: "pid, time"
            try:
                _, time_str = rest.split(",", 1)
                time_ns = parse_time(time_str.strip())
            except Exception:
                continue

            groups[label].append(time_ns)

    for label, times in groups.items():
        avg = sum(times) / len(times)
        med = statistics.median(times)
        print(f"{label}: avg={avg:.2f}ns, median={med:.2f}ns")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: script.py <input-file>")
        sys.exit(1)
    main(sys.argv[1])

