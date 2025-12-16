"""
Day 12: Christmas Tree Farm

https://adventofcode.com/2025/day/12
"""
__all__ = ('Day12',)

from typing import ClassVar

from ..solution import Solution

class Day12(Solution):
    NAME: ClassVar[str] = 'Christmas Tree Farm'

    def part_one(self, inp: str) -> int:
        *shapes, regions = inp.split('\n\n')
        shape_sizes = [shape.count('#') for shape in shapes]

        count = 0

        # solving packing is really difficult
        # but all generated inputs for AOC allow us to assume that
        # as long as sum of number of `#` all used shapes take up fit within `width * height`
        # we can fit them in (since for these cases the grids are way bigger than needed),
        # and IF NOT, we can assume it doesn't fit...
        for region in regions.splitlines():
            dims, required_shapes = region.split(':')
            w, h = map(int, dims.split('x'))

            # +1 if sum <= total region area
            count += sum(
                # (# of cells occupied by the i-th shape shape) * (count of this shape)
                shape_sizes[i] * int(count)
                for i, count in enumerate(required_shapes.split())
            ) <= w * h
        return count

    def part_one_2(self, inp: str) -> int:
        """Alternative implementation for part one by allocating a 3x3 block for any shape.

        and checking if the # of 3x3 blocks in the region >= the # of shapes.
        Therefore, the actual individual shapes do not matter.

        unoptimal packing, but it doesn't matter...
        """
        *_, regions = inp.split('\n\n')
        total = 0

        for region in regions.splitlines():
            dims, required_shapes = region.split(':')
            w, h = map(int, dims.split('x'))

            total += (w // 3) * (h // 3) >= sum(int(count) for count in required_shapes.split())
        return total

    def part_two(self, inp: str) -> None:
        """No part 2 for day 12!

        Happy Holidays!
        """

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))

        assert p1 == 408