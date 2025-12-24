"""
Day 5: Cafeteria

https://adventofcode.com/2025/day/5
"""
__all__ = ('Day5',)

from typing import ClassVar

from ..solution import Solution

class Day5(Solution):
    NAME: ClassVar[str] = 'Cafeteria'

    def part_one(self, inp: str) -> int:
        ranges, ingredients = inp.split('\n\n')
        ranges = [
            tuple(int(p) for p in r.split('-'))
            for r in ranges.splitlines()
        ]

        return sum(
            any(a <= int(ingredient) <= b for a, b in ranges)
            for ingredient in ingredients.splitlines()
        )

    def part_two(self, inp: str) -> int:
        # sort ranges so overlapping ranges will be found consecutively
        ranges = sorted(
            tuple(int(p) for p in r.split('-'))
            for r in inp.split('\n\n')[0].splitlines()
        )

        i = 1
        # merge overlapping ranges together (union)
        while i < len(ranges):
            a0, b0 = ranges[i - 1]
            a, b = ranges[i]

            # intersection of [a0, b0] and [a, b]:
            # a0--a--b--b0
            # a0--a--b0--b
            # a0--b0--a--b
            if a0 <= a <= b0:
                # union [a0, b0] and [a, b] into ranges[i - 1]
                ranges[i - 1] = (a0, max(b, b0))
                del ranges[i]
            else:
                # only traverse forwards if current node not deleted
                # otherwise, no need, since next el will get swapped back automatically during `del ranges[i]`
                i += 1

        # since all ranges are non-overlapping
        # we can just sum their numeric ranges
        return sum(b - a + 1 for a, b in ranges)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 874
        assert p2 == 348548952146313