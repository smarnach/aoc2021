#!/usr/bin/env python3

from functools import reduce
from statistics import median
import sys

CLOSING = {"(": ")", "[": "]", "{": "}", "<": ">"}
SYNTAX = {")": 3, "]": 57, "}": 1197, ">": 25137}
COMPLETE = {")": 1, "]": 2, "}": 3, ">": 4}

def main():
    syntax = 0
    complete = []
    for line in sys.stdin:
        stack = []
        for c in line.strip():
            if d := CLOSING.get(c):
                stack.append(d)
            else:
                if c != stack.pop():
                    syntax += SYNTAX[c]
                    break
        else:
            score = reduce(lambda x, c: 5 * x + COMPLETE[c], stack[::-1], 0)
            complete.append(score)
    print(syntax)
    print(median(complete))

if __name__ == "__main__":
    main()
