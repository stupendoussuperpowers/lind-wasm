#!/usr/bin/env python3

import sys
import re
import csv


def parse_time(s):
    if s.endswith("ns"):
        return float(s[:-2]) * 1e-9
    if s.endswith("µs") or s.endswith("us"):
        return float(s[:-2]) * 1e-6
    if s.endswith("ms"):
        return float(s[:-2]) * 1e-3
    if s.endswith("s"):
        return float(s[:-1])
    raise ValueError("Unknown time unit: " + s)


def format_time(seconds):
    if seconds < 1e-6:
        return f"{seconds*1e9:.3f}ns"
    if seconds < 1e-3:
        return f"{seconds*1e6:.3f}µs"
    if seconds < 1:
        return f"{seconds*1e3:.3f}ms"
    return f"{seconds:.3f}s"


def main():
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <inputfile>")
        sys.exit(1)

    infile = sys.argv[1]

    rows = {}

    with open(infile) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            entry, val = line.split(",")
            entry = entry.strip()
            val = val.strip()
            rows.setdefault(entry, []).append(parse_time(val))

    max_len = max(len(v) for v in rows.values())
    keys = sorted(rows.keys(), key=lambda x: int(x))

    with open(f"{sys.argv[1]}_out.csv", "w", newline="") as csvf:
        writer = csv.writer(csvf)

        # header
        writer.writerow(["Entry"] + keys)

        # avg
        avg_row = ["Avg"]
        for k in keys:
            avg = sum(rows[k]) / len(rows[k])
            avg_row.append(format_time(avg))
        writer.writerow(avg_row)

        # each sample row
        for i in range(0, max_len, 50_000):
            row = [i + 1]
            for k in keys:
                vals = rows[k]
                if i < len(vals):
                    row.append(format_time(vals[i]))
                else:
                    row.append("")
            writer.writerow(row)


if __name__ == "__main__":
    main()
