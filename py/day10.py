#!/usr/bin/env python3

from functools import reduce
from statistics import median
import re
import sys

SYNTAX = {")": 3, "]": 57, "}": 1197, ">": 25137}
COMPLETE = {"(": 1, "[": 2, "{": 3, "<": 4}

def main():
    syntax = 0
    complete = []
    for line in sys.stdin:
        n = 1
        while n:
            line, n = re.subn(r"\(\)|\[\]|\{\}|<>", "", line.strip())
        if corrupt := re.search(r"[])}>]", line):
            syntax += SYNTAX[corrupt[0]]
        else:
            score = reduce(lambda x, c: 5 * x + COMPLETE[c], line[::-1], 0)
            complete.append(score)
    print(syntax)
    print(median(complete))

if __name__ == "__main__":
    main()
