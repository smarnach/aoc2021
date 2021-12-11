#!/usr/bin/env python3

import numpy as np
import sys

def main():
    calls, _, boards = sys.stdin.read().partition("\n")
    calls = list(map(int, calls.split(",")))
    boards = np.array([
        [list(map(int, line.split())) for line in board.split("\n")]
        for board in boards.strip().split("\n\n")
    ])
    marks = np.zeros_like(boards, dtype=bool)
    score = lambda won: boards[won][~marks[won]].sum()
    won = None
    for c in calls:
        marks[boards == c] = True
        previous = won
        won = marks.all(axis=1).any(axis=1) | marks.all(axis=2).any(axis=1)
        if won.any() and not previous.any():
            print(score(won) * c)
        if won.all():
            print(score(won ^ previous) * c)
            break

if __name__ == "__main__":
    main()
