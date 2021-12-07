#!/usr/bin/env python3

from math import floor, ceil
from statistics import mean, median
import sys

def triangular(x):
    return x * (x + 1) // 2

def main():
    a = [int(x) for x in sys.stdin.read().split(",")]
    y = int(median(a))
    print(sum(abs(x - y) for x in a))
    z = mean(a)
    print(min(sum(triangular(abs(x - t)) for x in a) for t in [floor(z), ceil(z)]))

if __name__ == "__main__":
    main()
