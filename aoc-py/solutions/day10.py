"""
Day 10: Factory

https://adventofcode.com/2025/day/10
"""
__all__ = ('Day10',)

from typing import ClassVar, Iterator
from itertools import count, combinations_with_replacement

from z3 import Optimize, Ints

from ..solution import Solution

class Day10(Solution):
    NAME: ClassVar[str] = 'Factory'

    def _parse_input(self, inp: str) -> Iterator[tuple[list[str], list[tuple[int, ...]], list[int]]]:
        for row in inp.splitlines():
            lights, *schematics, joltages = row.split()

            lights = list(lights.strip('[]'))
            schematics = [tuple(int(x) for x in schematic.strip('()').split(',')) for schematic in schematics]
            joltages = [int(x) for x in joltages.strip(r'{}').split(',')]

            yield lights, schematics, joltages

    def _toggle(self, lights: list[str], schematic: tuple[int, ...]) -> list[str]:
        lights = lights[:]
        for index in schematic:
            lights[index] = '#' if lights[index] == '.' else '.'
        return lights

    def part_one(self, inp: str) -> int:
        total = 0

        for target, schematics, _ in self._parse_input(inp):
            # Note: brute force is fast enough for part one
            # try combinations of schematics of `n` schematics until we find one that works
            # this will be the minimal number of schematics (n) needed since we start at `n=1` and increment
            # use `_with_replacement` so we can reuse schematics
            for n in count(1):
                for combo in combinations_with_replacement(schematics, n):
                    lights = ['.'] * len(target)

                    for schematic in combo:
                        lights = self._toggle(lights, schematic)

                    if lights == target:
                        total += n
                        break
                else:
                    # if not broken out of inner loop, increase `n` and keep testing
                    continue
                # broken out of inner loop, break out of outer loop
                # move onto next row in input
                break
        return total

    def part_two(self, inp: str) -> int:
        total = 0

        for _, schematics, target in self._parse_input(inp):
            opt = Optimize()
            # `n{i}` represents the number of times schematic `i` is pulled
            vars = Ints(f'n{i}' for i in range(len(schematics)))
            # number of pulls must be non-negative
            for v in vars:
                opt.add(v >= 0)
            # for each target joltage
            for i, joltage in enumerate(target):
                # add the linear equation to the system
                opt.add(
                    # the equation itself is the sum of each schematic's contribution to the joltage
                    # this either 0 or 1 depending on if the index of the current joltage is included in the schematic
                    # this is multiplied against `n{i}` (vars[s]) to get the total contribution of that schematic to the joltage
                    # the sum of all schematic contributions must equal the target joltage
                    sum(
                        vars[s] for s, schematic in enumerate(schematics)
                        if i in schematic
                    ) == joltage
                )
            # we want to minimize the total number of pulls
            opt.minimize(sum(vars))
            opt.check()
            total += opt.model().eval(sum(vars)).as_long() # type: ignore[attr-defined]
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 481
        assert p2 == 20142