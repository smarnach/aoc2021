#!/usr/bin/env python3

from math import prod
from operator import gt, lt, eq
import sys

apply = lambda f: lambda v: f(*v)
OPERATORS = {0: sum, 1: prod, 2: min, 3: max, 5: apply(gt), 6: apply(lt), 7: apply(eq)}

class Bits:
    def __init__(self, s):
        self.bits = int(s, 16)
        self.pos = len(s) * 4
        self.version_sum = 0

    def get(self, n):
        self.pos -= n
        return (self.bits >> self.pos) & ((1 << n) - 1)

    def parse_packet(self):
        self.version_sum += self.get(3)
        type_id = self.get(3)
        if type_id == 4:
            result = 0
            while (d := self.get(5)) & 16:
                result = result * 16 + (d & 15)
            return result * 16 + d
        else:
            if self.get(1):
                values = [self.parse_packet() for _ in range(self.get(11))]
            else:
                target = -self.get(15) + self.pos
                values = []
                while self.pos > target:
                    values.append(self.parse_packet())
            return OPERATORS[type_id](values)

def main():
    bits = Bits(sys.stdin.read().strip())
    part2 = bits.parse_packet()
    print(bits.version_sum, part2, sep="\n")

if __name__ == "__main__":
    main()
