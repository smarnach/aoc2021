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

def overlap_count(counts, lines):
    counts.update(p for line in lines for p in line_points(*line))
    return sum(v > 1 for v in counts.values())

def main():
    lines = [tuple(map(int, re.findall("\d+", line))) for line in sys.stdin]
    straight = []
    diagonal = []
    for x0, y0, x1, y1 in lines:
        (straight if x0 == x1 or y0 == y1 else diagonal).append((x0, y0, x1, y1))
    counts = Counter()
    print(overlap_count(counts, straight))
    print(overlap_count(counts, diagonal))

if __name__ == "__main__":
    main()
