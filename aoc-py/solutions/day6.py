"""
Day 6: Trash Compactor

https://adventofcode.com/2025/day/6
"""
__all__ = ('Day6',)

from typing import ClassVar
from math import prod

from ..solution import Solution

class Day6(Solution):
    NAME: ClassVar[str] = 'Trash Compactor'

    def part_one(self, inp: str) -> int:
        grid = [row.split() for row in inp.splitlines()]
        n_rows = len(grid)
        n_cols = len(grid[0])

        return sum(
            (prod if grid[-1][j] == '*' else sum)([int(grid[i][j]) for i in range(n_rows - 1)])
            for j in range(n_cols)
        )

    def part_two(self, inp: str) -> int:
        grid = inp.splitlines()

        n_rows = len(grid)
        n_cols = max(len(row) for row in grid)

        total = 0
        curr = 0

        op = None

        # scan each column
        for j in range(n_cols):
            # my editor removes trailing spaces
            # so must account for some columns being shorter
            #
            # if a new operator is found at the bottom '*' or '+'
            # then add the previous sum/product `curr` onto `total`
            # reset `curr` to `1` if the new operator is mul else `0`
            if j < len(grid[-1]) and (new_op := grid[-1][j]) != ' ':
                op = new_op
                total += curr
                curr = op == '*'

            # for each column `j`
            # go down the rows... (stop before last row)
            # and record the digit in `num`
            #
            # again, we must consider a lack of trailing spaces
            # and manually record a space if out of bounds
            num = ''
            for i in range(n_rows - 1):
                num += grid[i][j] if j < len(grid[i]) else ' '

            try:
                # ensure we are able to convert to integer:
                # the only case we aren't is when it's all spaces (i.e. between expression blocks)
                # even ' 1   ' can be passed into int() which is why we can just record spaces
                num = int(num)

                if op == '*':
                    curr *= num
                else:
                    curr += num
            except ValueError:
                pass

        # since we only record a sum/product result when the next block is reached
        # the last block will never get added into `total`, so we must do it here
        return total + curr

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 3785892992137
        assert p2 == 7669802156452