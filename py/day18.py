#!/usr/bin/env python3

from ast import literal_eval
from copy import deepcopy
from functools import reduce
from itertools import islice, permutations
import sys

def walk(n, depth=1, parent=None, j=None):
    for i, x in enumerate(n):
        if isinstance(x, int):
            yield n, i, parent, j, depth
        else:
            yield from walk(x, depth + 1, n, i)

def explode(n):
    previous_m = previous_i = None
    it = walk(n)
    for m, i, parent, j, depth in it:
        if depth == 5:
            if previous_m is not None:
                previous_m[previous_i] += m[0]
            for next_m, next_i, _, _, _ in islice(it, 1, 2):
                next_m[next_i] += m[1]
            parent[j] = 0
            return True
        previous_m, previous_i = m, i
    return False

def split(n):
    for m, i, _, _, _ in walk(n):
        if m[i] > 9:
            m[i] = [m[i] // 2, (m[i] + 1) // 2]
            return True
    return False

def add(x, y):
    n = [deepcopy(x), deepcopy(y)]
    while explode(n) or split(n):
        pass
    return n

def magnitude(n):
    if isinstance(n, int):
        return n
    return 3 * magnitude(n[0]) + 2 * magnitude(n[1])

def main():
    numbers = [literal_eval(line) for line in sys.stdin]
    print(magnitude(reduce(add, numbers)))
    print(max(magnitude(add(x, y)) for x, y in permutations(numbers, 2)))

if __name__ == "__main__":
    main()
