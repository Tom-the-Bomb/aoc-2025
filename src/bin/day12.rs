//! Day 12: Christmas Tree Farm
//!
//! <https://adventofcode.com/2025/day/12>
use std::collections::VecDeque;
use aoc_2025::Solution;

pub struct Day12;

impl Solution for Day12 {
    const NAME: &'static str = "Christmas Tree Farm";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        let inp = inp.replace('\r', "");

        let mut parts = inp
            .split("\n\n")
            .collect::<VecDeque<_>>();

        let regions = parts
            .pop_back()
            .unwrap()
            .lines();

        let shape_sizes = parts
            .into_iter()
            .map(|shape| shape
                .bytes()
                .filter(|&c| c == b'#')
                .count()
            )
            .collect::<Vec<_>>();

        regions
            .filter(|region| {
                let (dims, required_shapes) = region.split_once(':').unwrap();
                let (w, h) = dims
                    .split_once('x')
                    .map(|(w, h)| (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap()))
                    .unwrap();

                required_shapes
                    .split_whitespace()
                    .enumerate()
                    .map(|(i, count)| shape_sizes[i] * count.parse::<usize>().unwrap())
                    .sum::<usize>() <= w * h
                })
            .count()
    }

    fn part_two(&self, _inp: &str) -> Self::OutputP2 {
        unimplemented!()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);

        println!("Part 1: {p1}");

        assert_eq!(p1, 408);
    }
}

fn main() {
    aoc_2025::run_day(12, &Day12);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}