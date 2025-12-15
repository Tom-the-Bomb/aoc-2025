"""
Day 12: Printing Department

https://adventofcode.com/2025/day/12
"""
__all__ = ('Day12',)

from typing import ClassVar

from ..solution import Solution

class Day12(Solution):
    NAME: ClassVar[str] = 'Printing Department'

    def part_one(self, inp: str) -> int:
        ...

    def part_two(self, inp: str) -> int:
        ...

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        # assert p1 == 12160
        # assert p2 == 92123