"""
Day 1: Secret Entrance

https://adventofcode.com/2025/day/1
"""
__all__ = ('Day1',)

from typing import ClassVar

from ..solution import Solution

class Day1(Solution):
    NAME: ClassVar[str] = 'Secret Entrance'

    def part_one(self, inp: str) -> int:
        count = 0
        dial = 50

        for line in inp.splitlines():
            value = int(line[1:])

            if line[0] == 'L':
                value *= -1

            dial += value
            dial %= 100

            if dial == 0:
                count += 1
        return count

    def part_two(self, inp: str) -> int:
        count = 0
        dial = 50

        for line in inp.splitlines():
            value = int(line[1:])

            if line[0] == 'L':
                value *= -1

            # edge case: ex: dial=0 and [L5]
            # should not add to count since `0` was reached last time
            # but -5 // 100 = 1 (not 0)
            if dial == 0 and value < 0:
                count -= 1

            dial += value

            zeroes, dial = divmod(dial, 100)
            count += abs(zeroes)

            # edge case: ex: dial=20 and [L20]
            # dial=0 now but 20 % 100 = 0 and we did pass by `0`
            if dial == 0 and value < 0:
                count += 1
        return count

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 1195
        assert p2 == 6770