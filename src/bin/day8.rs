//! Day 8: Playground
//!
//! <https://adventofcode.com/2025/day/8>
#![allow(future_incompatible)]

use std::{
    collections::{HashSet, HashMap},
    hash::Hash,
};
use itertools::Itertools;
use aoc_2025::Solution;

struct DisjointSet<'a, T>
where
    T: Eq + Hash,
{
    nodes: &'a [T],
    parents: HashMap<&'a T, &'a T>,
}

impl<'a, T> DisjointSet<'a, T>
where
    T: Eq + Hash,
{
    #[must_use]
    pub fn new(nodes: &'a [T]) -> Self {
        Self {
            nodes,
            parents: nodes
                .iter()
                .map(|node| (node, node))
                .collect::<HashMap<_, _>>()
        }
    }

    #[must_use]
    pub fn root(&mut self, node: &'a T) -> &'a T {
        if self.parents[node] == node {
            return node;
        }

        let root = self.root(self.parents[node]);
        self.parents.insert(node, root);
        root
    }

    #[must_use]
    pub fn get_sizes(&mut self) -> HashMap<&'a T, usize> {
        let mut sizes = HashMap::with_capacity(self.parents.len());

        for node in self.nodes {
            *sizes.entry(self.root(node))
                .or_insert(0) += 1;
        }
        sizes
    }

    #[must_use]
    pub fn count(&mut self) -> usize {
        self.nodes
            .iter()
            .map(|node| self.root(node)
        )
        .collect::<HashSet<_>>()
        .len()
    }

    pub fn union(&mut self, node1: &'a T, node2: &'a T) {
        let root1 = self.root(node1);
        let root2 = self.root(node2);

        self.parents.insert(root2, root1);
    }
}

type BoxT = [usize; 3];

pub struct Day8;

impl Day8 {
    #[must_use]
    #[inline]
    fn parse_input(inp: &str) -> (Vec<BoxT>, Vec<(BoxT, BoxT)>) {
        let boxes = inp
            .lines()
            .map(|line| {
                line
                    .split(',')
                    .map(|c| c.parse().unwrap())
                    .collect_array()
                    .unwrap()
            })
            .collect::<Vec<BoxT>>();

        let edges = boxes
            .iter()
            .copied()
            .tuple_combinations()
            .sorted_unstable_by_key(|(a, b)| a
                .iter()
                .zip(b.iter())
                .map(|(&x1, &x2)| x1.abs_diff(x2).pow(2))
                .sum::<usize>()
            )
            .collect::<Vec<_>>();

        (boxes, edges)
    }
}

impl Solution for Day8 {
    const NAME: &'static str = "Playground";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        let (boxes, edges) = Self::parse_input(inp);
        let mut sets = DisjointSet::new(&boxes);

        for (box1, box2) in edges.iter().take(1000) {
            sets.union(box1, box2);
        }

        sets
            .get_sizes()
            .values()
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(3)
            .product()
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        let (boxes, edges) = Self::parse_input(inp);
        let mut sets = DisjointSet::new(&boxes);

        let mut last_box1 = None;
        let mut last_box2 = None;

        for (box1, box2) in &edges {
            if sets.count() == 1 {
                break;
            }

            last_box1 = Some(box1);
            last_box2 = Some(box2);
            sets.union(box1, box2);
        }

        last_box1.unwrap()[0] * last_box2.unwrap()[0]
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 54_180);
        assert_eq!(p2, 25_325_968);
    }
}

fn main() {
    aoc_2025::run_day(8, &Day8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}