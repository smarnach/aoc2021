#!/usr/bin/env python3

from copy import deepcopy
from heapq import heappop, heappush
from itertools import dropwhile, takewhile
import sys

class Room:
    def __init__(self, type_, amphipods):
        self.type = type_
        self.amphipods = amphipods
        self.size = len(amphipods)
    def get(self):
        a = None
        for a in dropwhile(self.accepts, self.amphipods):
            pass
        return a, self.size - len(self.amphipods) + 1
    def traversable(self):
        return True
    def accepts(self, amphipod):
        return amphipod == self.type and all(a == self.type for a in self.amphipods)
    def remove(self):
        self.amphipods.pop()
    def add(self, amphipod):
        self.amphipods.append(amphipod)
        return self.size - len(self.amphipods) + 1
    def dest_dist(self):
        return self.size - len(list(takewhile(self.accepts, self.amphipods))) + 1
    def min_cost(self, dest_dist, _index):
        cost = 0
        for i, a in dropwhile(lambda x: self.accepts(x[1]), enumerate(self.amphipods)):
            if a == self.type:
                cost += (self.size - i + dest_dist[a] + 2) * 10 ** a
            else:
                cost += (2 * abs(a - self.type) + self.size - i + dest_dist[a]) * 10 ** a
            dest_dist[a] -= 1
        return cost
    def key(self):
        return tuple(self.amphipods)
    def __repr__(self):
        return "".join(chr(65 + a) for a in self.amphipods)

class Cell:
    def __init__(self):
        self.amphipod = None
    def get(self):
        return self.amphipod, 0
    def traversable(self):
        return self.amphipod is None
    def accepts(self, amphipod):
        return self.amphipod is None
    def remove(self):
        self.amphipod = None
    def add(self, amphipod):
        self.amphipod = amphipod
        return 0
    def min_cost(self, dest_dist, index):
        a = self.amphipod
        if a is None:
            return 0
        else:
            cost = (abs(index - (a * 2 + 2)) + dest_dist[a]) * 10 ** a
            dest_dist[a] -= 1
            return cost
    def key(self):
        return self.amphipod
    def __repr__(self):
        return chr(65 + self.amphipod) if self.amphipod else " "

class Burrow:
    def __init__(self, rooms):
        room = (Room(*x) for x in enumerate(rooms))
        self.state = [
            Cell(), Cell(),
            next(room), Cell(), next(room), Cell(),
            next(room), Cell(), next(room),
            Cell(), Cell()
        ]
    def moves(self):
        for i, x in enumerate(self.state):
            amphipod, source_dist = x.get()
            if amphipod is None:
                continue
            for j, y in enumerate(self.state):
                if (i == j or type(x) == type(y) or not y.accepts(amphipod) or
                    not all(self.state[k].traversable() for k in range(min(i, j) + 1, max(i, j)))):
                    continue
                new = deepcopy(self)
                new.state[i].remove()
                dest_dist = new.state[j].add(amphipod)
                cost = (source_dist + abs(i - j) + dest_dist) * 10 ** amphipod
                yield cost, new
    def key(self):
        return tuple(x.key() for x in self.state)
    def min_cost(self):
        cost = 0
        dest_dist = [room.dest_dist() for room in self.state[2:9:2]]
        for i, x in enumerate(self.state):
            cost += x.min_cost(dest_dist, i)
        return cost
    def __lt__(self, _other):
        return False
    def __repr__(self):
        return "|{}|".format("|".join(repr(x) for x in self.state))
    def find_cheapest(self):
        queue = [(self.min_cost(), 0, self)]
        seen = set()
        while queue:
            estimate, cost, burrow = heappop(queue)
            if estimate == cost:
                return cost
            if (key := burrow.key()) in seen:
                continue
            seen.add(key)
            for additional_cost, new in burrow.moves():
                new_cost = cost + additional_cost
                heappush(queue, (new_cost + new.min_cost(), new_cost, new))

def main():
    burrow = Burrow([[3, 3], [0, 2], [0, 1], [1, 2]])
    print(burrow.find_cheapest())
    burrow = Burrow([[3, 3, 3, 3], [0, 1, 2, 2], [0, 0, 1, 1], [1, 2, 0, 2]])
    print(burrow.find_cheapest())

if __name__ == "__main__":
    main()
