"""
Day 4: Printing Department

https://adventofcode.com/2025/day/4
"""
__all__ = ('Day4',)

from typing import ClassVar

from ..solution import Solution
from ..utils import neighbors_4, neighbors_diag

class Day4(Solution):
    NAME: ClassVar[str] = 'Printing Department'

    def part_one(self, inp: str) -> int:
        grid = inp.splitlines()
        count = 0

        n_rows = len(grid)
        n_cols = len(grid[0])

        return sum(
            sum(
                i in range(n_rows) and j in range(n_cols) and grid[i][j] == '@'
                for i, j in neighbors_4(i, j) + neighbors_diag(i, j)
            ) < 4
            for i, row in enumerate(grid)
            for j, cell in enumerate(row)
            if cell == '@'
        )

    def part_two(self, inp: str) -> int:
        grid = [list(row) for row in inp.splitlines()]
        count = 0

        n_rows = len(grid)
        n_cols = len(grid[0])

        while True:
            accessible = 0

            for i, row in enumerate(grid):
                for j, cell in enumerate(row):
                    if cell == '@' and sum(
                        i in range(n_rows) and j in range(n_cols) and grid[i][j] == '@'
                        for i, j in neighbors_4(i, j) + neighbors_diag(i, j)
                    ) < 4:
                        grid[i][j] = '.'
                        accessible += 1

            if accessible == 0:
                break

            count += accessible
        return count

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 1460
        assert p2 == 9243