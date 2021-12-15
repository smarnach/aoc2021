#!/usr/bin/env python3

import numpy as np
import sys

def low_risk_path(a):
    b = np.full_like(a, np.iinfo(int).max - 9)
    b[0, 0] = 0
    for i in range(2 * b.shape[0]):
        for c, d in [(a, b), (a.T, b.T)]:
            d[:-1] = np.minimum(d[:-1], d[1:] + c[:-1])
            d[1:] = np.minimum(d[1:], d[:-1] + c[1:])
    return b[-1, -1]

def main():
    a = np.array([[int(c) for c in line.strip()] for line in sys.stdin])
    print(low_risk_path(a))
    a = np.block([[(a + i + j - 1) % 9 + 1 for j in range(5)] for i in range(5)])
    print(low_risk_path(a))

if __name__ == "__main__":
    main()
