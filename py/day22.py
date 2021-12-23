#!/usr/bin/env python3

import numpy as np
import re
import sys

def main():
    input = sys.stdin.read()
    cuboids = [int(x) for x in re.findall(r"-?\d+", input)]
    cuboids = np.array(cuboids).reshape((len(cuboids) // 6, 3, 2))
    cuboids[:, :, 1] += 1
    actions = [line.split()[0] == "on" for line in input.splitlines()]
    coords = [np.array(sorted(set(x.ravel()))) for x in cuboids.swapaxes(0, 1)]
    diffs = [np.r_[np.diff(c), 0] for c in coords]
    volumes = diffs[0][:, None, None] * diffs[1][:, None] * diffs[2]
    state = np.zeros(tuple(len(c) for c in coords), bool)
    for i, (action, cuboid) in enumerate(zip(actions, cuboids)):
        if i == 20:
            print(volumes[state].sum())
        state[tuple(slice(*np.searchsorted(*c)) for c in zip(coords, cuboid))] = action
    print(volumes[state].sum())

if __name__ == "__main__":
    main()
