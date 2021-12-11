#!/usr/bin/env python3

import numpy as np
from scipy import ndimage
import sys

KERNEL = np.ones((3, 3), int)

def step(a):
    a += 1
    b = a
    for dummy in range(20):
        f = b > 9
        b = a + ndimage.convolve(f.astype(int), KERNEL, mode="constant")
    a[:] = b
    a[f] = 0
    return f.sum()

def main():
    a = np.array([[int(c) for c in line.strip()] for line in sys.stdin])
    print(sum(step(a) for dummy in range(100)))
    time = 101
    while step(a) != 100:
        time += 1
    print(time)

if __name__ == "__main__":
    main()
