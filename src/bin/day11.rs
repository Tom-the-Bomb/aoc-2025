//! Day 11: Reactor
//!
//! <https://adventofcode.com/2025/day/11>
use std::collections::HashMap;
use aoc_2025::Solution;

pub struct Day11;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

impl Day11 {
    #[must_use]
    #[inline]
    fn parse_input(inp: &str) -> Graph<'_> {
        inp
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(':').unwrap();

                (from, to.split_whitespace().collect())
            })
            .collect()
    }

    #[must_use]
    fn dfs_1<'a>(cache: &mut HashMap<&'a str, usize>, graph: &Graph<'a>, node: &'a str) -> usize {
        if let Some(&res) = cache.get(node) {
            return res;
        }

        if node == "out" {
            return 1;
        }

        let result = graph
            .get(node)
            .unwrap()
            .iter()
            .map(|&child| Self::dfs_1(cache, graph, child))
            .sum::<usize>();

        cache.insert(node, result);
        result
    }

    #[must_use]
    fn dfs_2<'a>(
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        graph: &Graph<'a>,
        node: &'a str,
        mut dac: bool,
        mut fft: bool,
    ) -> usize {
        let args = (node, dac, fft);

        if let Some(&res) = cache.get(&args) {
            return res;
        }

        match node {
            "out" if dac && fft => return 1,
            "out" => return 0,
            "dac" => dac = true,
            "fft" => fft = true,
            _ => (),
        }

        let result = graph
            .get(node)
            .unwrap()
            .iter()
            .map(|&child| Self::dfs_2(cache, graph, child, dac, fft))
            .sum::<usize>();

        cache.insert(args, result);
        result
    }
}

impl Solution for Day11 {
    const NAME: &'static str = "Reactor";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        let graph = Self::parse_input(inp);
        let mut cache = HashMap::new();
        Self::dfs_1(&mut cache, &graph, "you")
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        let graph = Self::parse_input(inp);
        let mut cache = HashMap::new();
        Self::dfs_2(&mut cache, &graph, "svr", false, false)
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 758);
        assert_eq!(p2, 490_695_961_032_000);
    }
}

fn main() {
    aoc_2025::run_day(11, &Day11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}