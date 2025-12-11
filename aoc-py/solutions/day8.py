"""
Day 8: Playground

https://adventofcode.com/2025/day/8
"""
from __future__ import annotations

__all__ = ('Day8',)

from typing import ClassVar

from math import dist
from collections import defaultdict
from itertools import combinations

from ..solution import Solution

class DisjointSet[T]:
    def __init__(self, nodes: list[T]) -> None:
        self._nodes = nodes
        # we will use a tree to represent a set
        #
        # maps `node` -> the direct parent of `node`
        # start off with all nodes being disjoint sets (sets containing just itself, so its parent will also be itself)
        # i.e. given nodes [1, 2, 3] we will re-organize them with {1:1, 2:2, 3:3}
        self._parents = {box: box for box in nodes}

    def root(self, node: T) -> T:
        """Finds the root node of the set containing `node`, a unique identifier representing a disjoint set"""
        # if the current node's parent is itself
        # then we are at the root just like how we defined it in the constructor
        if self._parents[node] == node:
            return node

        # optimization: path compression
        # if we find that our tree is: 3->4->5->...->root
        # we can just re-declare 3's parent to be 5 (the parent of 4 (the parent of 3))
        # eventually, due to recursion, the path will compress it such that all nodes (i.e. 3) will be connected to the root directly
        # reducing the depth of the tree (since `root` is recursive and is slower if the tree is deeper)
        self._parents[node] = self.root(self._parents[node])
        # now that our parent is our root, we can just return our parent
        return self._parents[node]

    def get_sizes(self) -> dict[T, int]:
        """Returns a mapping of `root each set` -> the size of the set

        since a set's root is a unique identifier for the set
        """
        sizes = defaultdict(int)

        for node in self._nodes:
            sizes[self.root(node)] += 1
        return sizes

    def count(self) -> int:
        """Returns the number of disjoint sets in this structure"""

        # check the size of the set containing the roots (identifier) for each disjoint set
        return len({self.root(node) for node in self._nodes})

    def union(self, node1: T, node2: T) -> None:
        """Unions (merges) 2 disjoint sets together (into 1 "joint" set)

        Specifically, we are merging the set containing `node2` into the set containing `node1`
        """
        # find the root of `node2`, and make it a child of the root of `node1`
        # i.e. declare that the parent of the root of `node2` is the root of `node1`
        self._parents[self.root(node2)] = self.root(node1)

class Day8(Solution):
    NAME: ClassVar[str] = 'Playground'

    def part_one(self, inp: str) -> int:
        boxes = [
            tuple(int(a) for a in box.split(','))
            for box in inp.splitlines()
        ]
        sets = DisjointSet(boxes)
        # find all possible pairs of boxes, and sort them by their distance (edge length, low->high)
        edges = sorted(combinations(boxes, 2), key=lambda edge: dist(edge[0], edge[1]))

        # process the 1000 shortest edges
        for box1, box2 in edges[:1000]:
            sets.union(box1, box2)

        sizes = sorted(sets.get_sizes().values(), reverse=True)
        # product of the sizes of the 3 largest sets
        return sizes[0] * sizes[1] * sizes[2]

    def part_two(self, inp: str) -> int:
        boxes = [
            tuple(int(a) for a in box.split(','))
            for box in inp.splitlines()
        ]
        sets = DisjointSet(boxes)
        # find all possible pairs of boxes, and sort them by their distance (edge length, low->high)
        edges = sorted(combinations(boxes, 2), key=lambda edge: dist(edge[0], edge[1]))

        last_box1 = None
        last_box2 = None

        for box1, box2 in edges:
            # process edges until everything is inside 1 connected set (tree)
            if sets.count() == 1:
                break
            # record the last pair of boxes that were processed before we stop
            last_box1 = box1
            last_box2 = box2
            sets.union(box1, box2)

        assert last_box1 and last_box2
        return last_box1[0] * last_box2[0]

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 54180
        assert p2 == 25325968