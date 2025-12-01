//! Day 1: Secret Entrance
//!
//! <https://adventofcode.com/2025/day/1>
use std::fmt::Display;
use aoc_2025::Solution;

pub struct Day1;

impl Solution for Day1 {
    const NAME: &'static str = "Secret Entrance";

    fn part_one<T: Display>(&self, inp: T) -> Self::OutputP1 {
        let mut dial = 50;

        inp
            .to_string()
            .lines()
            .filter(|line| {
                let (op, val) = line.split_at(1);
                let mut val = val.parse::<isize>().unwrap();

                if op == "L" { val *= -1; }

                dial += val;
                dial = dial.rem_euclid(100);

                dial == 0
            })
            .count()
    }

    fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
        let mut dial = 50;

        inp
            .to_string()
            .lines()
            .map(|line| {
                let (op, val) = line.split_at(1);
                let mut val = val.parse::<isize>().unwrap();

                if op == "L" { val *= -1; }

                let mut zeroes = if dial == 0 && val < 0 { -1 } else { 0 };
                dial += val;

                zeroes += dial.div_euclid(100).abs();
                dial = dial.rem_euclid(100);

                if dial == 0 && val < 0 { zeroes += 1; }

                zeroes
            })
            .sum::<isize>().cast_unsigned()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1195);
        assert_eq!(p2, 6770);
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