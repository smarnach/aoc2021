#!/usr/bin/env python3

import numpy as np
import sys

def main():
    school = np.bincount(sys.stdin.read().split(","))
    school.resize((9, 1))
    a = np.matrix(np.eye(9, k=1, dtype=int))
    a[[6, 8], 0] = 1
    print((a ** 80 @ school).sum())
    print((a ** 256 @ school).sum())

if __name__ == "__main__":
    main()
