#!/usr/bin/env python3

import numpy as np
import re
import sys

def main():
    x0, x1, y0, y1 = map(int, re.findall(r"-?\d+", sys.stdin.read()))
    print(y0 * (y0 + 1) // 2)
    vx = np.arange(1, x1 + 1)
    vy = np.arange(y0, -y0)
    target_x = np.array([[x0], [x1 + 1]])
    target_y = np.array([[y1], [y0 - 1]])
    tx = np.ceil(vx + 0.5 - np.sqrt((vx + 0.5) ** 2 - 2 * target_x))
    ty = np.ceil(vy + 0.5 + np.sqrt((vy + 0.5) ** 2 - 2 * target_y))
    t0 = np.maximum(tx[0, :, np.newaxis], ty[0])
    t1 = np.fmin(tx[1, :, np.newaxis], ty[1])
    print((t1 > t0).sum())

if __name__ == "__main__":
    np.seterr("ignore")
    main()
