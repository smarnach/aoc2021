#!/usr/bin/env python3

from itertools import count
from functools import lru_cache
import numpy as np
import sys

def deterministic(pos0, pos1):
    score0 = score1 = 0
    for die in count(5, 9):
        new_pos0 = (pos0 + die) % 10 + 1
        score0, pos0, score1, pos1 = score1, pos1, score0 + new_pos0, new_pos0
        if score1 >= 1000:
            return (die + 4) // 3 * score0

@lru_cache(maxsize=None)
def dirac_wins(score0, pos0, score1, pos1):
    if score1 >= 21:
        return np.array([0, 1])
    result = np.array([0, 0])
    for die, count in zip(range(3, 10), [1, 3, 6, 7, 6, 3, 1]):
        new_pos0 = (pos0 + die - 1) % 10 + 1
        result += count * dirac_wins(score1, pos1, score0 + new_pos0, new_pos0)[::-1]
    return result

def main():
    pos0, pos1 = [int(line.split()[-1]) for line in sys.stdin]
    print(deterministic(pos0, pos1))
    print(max(dirac_wins(0, pos0, 0, pos1)))

if __name__ == "__main__":
    main()
