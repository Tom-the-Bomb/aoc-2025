//! Day 3: Lobby
//!
//! <https://adventofcode.com/2025/day/3>
use aoc_2025::Solution;

pub struct Day3;

impl Day3 {
    #[must_use]
    fn compute_max(inp: &str, n: usize) -> usize {
        inp
            .lines()
            .flat_map(|line| {
                let line = line.as_bytes();
                let mut max_i = 0;

                (0..n).rev()
                    .map(move |exp| {
                        if exp < n - 1 {
                            max_i += 1;
                        }

                        #[allow(clippy::mut_range_bound)]
                        for i in max_i..line.len() - exp {
                            if line[i] > line[max_i] {
                                max_i = i;
                            }
                        }
                        (line[max_i] as char).to_digit(10).unwrap() as usize
                            * (10usize).pow(u32::try_from(exp).unwrap())
                    })
            })
            .sum::<usize>()
    }
}

impl Solution for Day3 {
    const NAME: &'static str = "Lobby";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        Self::compute_max(inp, 2)
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        Self::compute_max(inp, 12)
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 17_196);
        assert_eq!(p2, 171_039_099_596_062);
    }
}

fn main() {
    aoc_2025::run_day(3, &Day3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}