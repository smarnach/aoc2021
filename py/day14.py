#!/usr/bin/env python3.10

from collections import Counter
from itertools import pairwise
import sys

def main():
    start, rules = sys.stdin.read().split("\n\n")
    rules = dict(line.split(" -> ") for line in rules.splitlines())
    pairs = Counter(x + y for x, y in pairwise(start))
    for steps in [10, 30]:
        for _ in range(steps):
            new = dict.fromkeys(rules, 0)
            for p, c in pairs.items():
                new[p[0] + rules[p]] += c
                new[rules[p] + p[1]] += c
            pairs = new
        counts = Counter(start[-1])
        for (x, _), c in pairs.items():
            counts.update({x: c})
        print(max(counts.values()) - min(counts.values()))

if __name__ == "__main__":
    main()
