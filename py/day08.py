#!/usr/bin/env python3

from collections import Counter
from functools import reduce
import sys

KEY = {42: 0, 17: 1, 34: 2, 39: 3, 30: 4, 37: 5, 41: 6, 25: 7, 49: 8, 45: 9}

def main():
    easy_digits = 0
    total = 0
    for line in sys.stdin:
        patterns, _, digits = line.partition("|")
        c = Counter(patterns)
        decoded = [KEY[sum(c[x] for x in d)] for d in digits.split()]
        easy_digits += sum(d in [1, 4, 7, 8] for d in decoded)
        total += reduce(lambda x, y: 10 * x + y, decoded)
    print(easy_digits)
    print(total)

if __name__ == "__main__":
    main()
