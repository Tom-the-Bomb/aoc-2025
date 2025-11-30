"""
Day 1: Historian Hysteria

https://adventofcode.com/2025/day/1
"""
__all__ = ('Day1',)

from typing import ClassVar

from ..solution import Solution

class Day1(Solution):
    NAME: ClassVar[str] = ''

    def part_one(self, inp: str) -> int:
        ...

    def part_two(self, inp: str) -> int:
        ...

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == ...
        assert p2 == ...