#!/usr/bin/env python3

from itertools import takewhile
import numpy as np
import sys

def fold(dots, line):
    axis, x = line.split("=", 1)
    x = int(x)
    c = dots[:, "xy".index(axis[-1])]
    c[c > x] = 2 * x - c[c > x]

def main():
    dots = np.array([
        [int(x) for x in line.strip().split(",", 1)]
        for line in takewhile(str.strip, sys.stdin)
    ])
    fold(dots, next(sys.stdin))
    print(len(set(map(tuple, dots))))
    for line in sys.stdin:
        fold(dots, line)
    code = np.full(dots.max(axis=0) + 1, " ")
    code[tuple(dots.T)] = "█"
    print("\n".join(map("".join, code.T)))

if __name__ == "__main__":
    main()
