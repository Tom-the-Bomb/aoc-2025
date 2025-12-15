"""
Day 11: Printing Department

https://adventofcode.com/2025/day/11
"""
__all__ = ('Day11',)

from typing import ClassVar
from functools import cache

from ..solution import Solution

class Day11(Solution):
    NAME: ClassVar[str] = 'Printing Department'
    # we must store the graph as an attribute
    # so that the DFS functions can access it without needing to pass it around
    # this also allows us to cache the arguments (since graphs which are dictionaries are not hashable)
    graph: dict[str, list[str]]

    def _parse_input(self, inp: str) -> dict[str, list[str]]:
        return {
            (parts := line.split(':'))[0]: parts[1].split()
            for line in inp.splitlines()
        }

    @cache
    def dfs_1(self, node: str) -> int:
        # base case: reached end of tree: the 'out' node, return 1 as we have completed 1 way to reach here
        if node == 'out':
            return 1
        # recursively DFS all paths to 'out' nodes
        # and sum the number of ways to them (AKA count the number of times the base-case is reached, summing up the 1s each time)
        return sum(self.dfs_1(child) for child in self.graph.get(node, []))

    @cache
    def dfs_2(self, node: str, dac: bool = False, fft: bool = False) -> int:
        # similar to `dfs` used in part 1, but now we need to ensure the nodes 'dac' and 'fft' have been visited
        # this is indicated through the booleans `dac` and `fft`
        match node:
            # return 1: valid path as we reached 'out' having visited both 'dac' and 'fft'
            case 'out' if dac and fft:
                return 1
            # return 0: this is not a valid path as we reached 'out' without visiting both 'dac' and 'fft'
            # but we still need to return to finish DFS
            case 'out':
                return 0
            # if we reach 'dac' or 'fft', set the corresponding boolean to True
            # and we continue the DFS with the variable set to True for the rest of the path
            case 'dac':
                dac = True
            case 'fft':
                fft = True

        return sum(self.dfs_2(child, dac, fft) for child in self.graph.get(node, []))

    def part_one(self, inp: str) -> int:
        self.graph = self._parse_input(inp)
        return self.dfs_1('you')

    def part_two(self, inp: str) -> int:
        self.graph = self._parse_input(inp)
        return self.dfs_2('svr')

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 758
        assert p2 == 490695961032000