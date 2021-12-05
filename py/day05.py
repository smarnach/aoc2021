#!/usr/bin/env python3

from collections import Counter
import re
import sys

def line_points(x0, y0, x1, y1):
    dx = (x1 > x0) - (x1 < x0)
    dy = (y1 > y0) - (y1 < y0)
    while True:
        yield x0, y0
        if (x0, y0) == (x1, y1):
            break
        x0 += dx
        y0 += dy

def overlap_count(lines):
    counts = Counter(p for line in lines for p in line_points(*line))
    return sum(v > 1 for v in counts.values())

def main():
    lines = [tuple(map(int, re.findall("\d+", line))) for line in sys.stdin]
    straight = [(x0, y0, x1, y1) for x0, y0, x1, y1 in lines if x0 == x1 or y0 == y1]
    print(overlap_count(straight))
    print(overlap_count(lines))

if __name__ == "__main__":
    main()
