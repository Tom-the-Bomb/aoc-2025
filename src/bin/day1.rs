//! Day 1: ...
//!
//! <https://adventofcode.com/2025/day/1>
use std::fmt::Display;
use aoc_2025::Solution;

pub struct Day1;

impl Solution for Day1 {
    const NAME: &'static str = "";

    fn part_one<T: Display>(&self, inp: T) -> Self::OutputP1 {
        todo!()
    }

    fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
        todo!()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, todo!());
        assert_eq!(p2, todo!());
    }
}

fn main() {
    aoc_2025::run_day(1, &Day1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}