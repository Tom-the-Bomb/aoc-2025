"""
Day 2: Gift Shop

https://adventofcode.com/2025/day/2
"""
__all__ = ('Day2',)

from typing import ClassVar, Iterator
import re

from ..solution import Solution

class Day2(Solution):
    NAME: ClassVar[str] = 'Gift Shop'

    def _get_ids(self, inp: str) -> Iterator[int]:
        """A generator that is an iterator over every single ID in the union of all ranges from parsing the raw input"""
        ranges = inp.split(',')

        for rng in ranges:
            a, b = map(int, rng.split('-', maxsplit=1))

            for id_ in range(a, b + 1):
                yield id_

    def part_one(self, inp: str) -> int:
        invalid = 0

        for id_ in self._get_ids(inp):
            str_id = str(id_)

            # checks if first half of string is equal to second half of string
            # then the ID is invalid
            mid = len(str_id) // 2

            if str_id[:mid] == str_id[mid:]:
                invalid += id_
        return invalid

    def part_two(self, inp: str) -> int:
        invalid = 0

        for id_ in self._get_ids(inp):
            str_id = str(id_)

            # captures a single numeric string (of at least length 1) as the first group: `([0-9]+)`
            # `\1+` requires at least 1 other match that is identical to the first group (above)
            # resulting in matching the repetition of a numeric string (number)
            if re.fullmatch(r'([0-9]+)\1+', str_id):
                invalid += id_
        return invalid

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 43952536386
        assert p2 == 54486209192