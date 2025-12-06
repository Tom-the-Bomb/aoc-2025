"""
Day 3: Lobby

https://adventofcode.com/2025/day/3
"""
__all__ = ('Day3',)

from typing import ClassVar

from ..solution import Solution

class Day3(Solution):
    NAME: ClassVar[str] = 'Lobby'

    # finds the sum of the max joltages of length `n``
    def _compute_max(self, inp: str, n: int) -> int:
        total = 0
        for line in inp.splitlines():
            # indicates the index of the last digit
            # starts at `-1` because we want to start at `0`
            # as we later add 1 to `max_i` because we want to start right AFTER (not including) `max_i`
            max_i = -1

            # find each digit
            # `exp` represents the magnitude of the digit
            # i.e. if we want a 12 digit joltage, we need to start at `10^11` => `10^(n - 1)`
            # 11 zeroes gives us 12 digits => `n - 1` zeroes gives us `n` digits
            #
            # the last digit will be exp=0 => magnitude of `1`
            for exp in range(n - 1, -1, -1):
                max_i += 1
                # stop at `exp` spots before the end
                # to save `exp` spots for the remaining `exp` digits we still need to find after this one
                for i in range(max_i, len(line) - exp):
                    if line[i] > line[max_i]:
                        # find index of maximum in the given range
                        # we can then use `max_i` for our next digit to know when to start
                        max_i = i

                total += int(line[max_i]) * 10 ** exp
        return total

    def _old_part_one(self, inp: str) -> int:
        total = 0
        for line in inp.splitlines():
            # find the global maximum digit in `line`
            #
            # store index since we need it to know when to start later
            max_i = 0
            for i in range(len(line) - 1):
                if line[i] > line[max_i]:
                    max_i = i

            # start after where the max value was found (`max_i`)
            # maximum digit that occurs after the global max
            #
            # store value directly since index is useless
            max_2 = '0'
            for i in range(max_i + 1, len(line)):
                if line[i] > max_2:
                    max_2 = line[i]

            total += int(line[max_i]) * 10 + int(max_2)
        return total

    def part_one(self, inp: str) -> int:
        return self._compute_max(inp, 2)

    def part_two(self, inp: str) -> int:
        return self._compute_max(inp, 12)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 17196
        assert p2 == 171039099596062