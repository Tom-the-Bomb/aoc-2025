"""
Day 7: Laboratories

https://adventofcode.com/2025/day/7
"""
__all__ = ('Day7',)

from typing import ClassVar

from ..solution import Solution

class Day7(Solution):
    NAME: ClassVar[str] = 'Laboratories'

    def part_one(self, inp: str) -> int:
        grid = [list(row) for row in inp.splitlines()]

        n_rows = len(grid)
        n_cols = len(grid[0])

        split = 0

        for i in range(1, n_rows):
            row = grid[i]

            for j in range(n_cols):
                if grid[i - 1][j] in ('S', '|'):
                    if row[j] == '.':
                        row[j] = '|'
                    elif row[j] == '^':
                        split += 1

                        if j > 0:
                            row[j - 1] = '|'
                        if j < n_cols:
                            row[j + 1] = '|'
        return split

    def part_two(self, inp: str) -> int:
        # using the same simulation algorithm as P1
        # instead we replace '.' and '|' with the (# of paths) from S to get to the current cell
        # i.e. empty cells '.' are never reached they will be 0
        # once reached a '|' will be a positive integer
        # use `-1` to represent splitters
        #
        # we will use the Pascal's triangle (binomial coefficient) unique paths counting algorithm
        # the (# of paths) to the current cell is the sum of the (# of paths) that reach this cell from the above row
        # i.e.
        # .|.|..| is actually .1.1..1
        # |^|^|.|             1^2^1.1
        grid = [
            [
                0 if cell == '.'
                else 1 if cell == 'S'
                else -1 for cell in row
            ]
            for row in inp.splitlines()
        ]

        n_rows = len(grid)
        n_cols = len(grid[0])

        for i in range(1, n_rows):
            row = grid[i]

            for j in range(n_cols):
                prev = grid[i - 1][j]

                # if previously there is no beam
                # then we will still have no beam...
                if prev > 0:
                    # if we are at a splitter
                    if row[j] == -1:
                        # we can propagate the # of paths onto the cells to the left and right (add them)
                        # we can guarantee that a splitter's (left/right) won't be other splitters
                        # if they are empty their value is `0` so we just add it on
                        # if they are not-empty and have been traversed to already (>0) i.e. from another splitter adjacent to it
                        # we just add onto the value of our beam (Pascal's triangle counting)
                        if j > 0:
                            row[j - 1] += prev
                        if j < n_cols:
                            row[j + 1] += prev
                    else:
                        # simply just propogate the # of paths from the same beam above
                        row[j] += prev

        # simply sum the (# of paths) to each final cell to get the total # of paths to the end
        return sum(grid[-1])

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 1630
        assert p2 == 47857642990160