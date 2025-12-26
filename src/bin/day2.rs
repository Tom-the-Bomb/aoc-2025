//! Day 2: Gift Shop
//!
//! <https://adventofcode.com/2025/day/2>
use std::sync::LazyLock;
use fancy_regex::Regex;
use aoc_2025::Solution;

static PAT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([0-9]+)\1+$").unwrap());

pub struct Day2;

impl Day2 {
    #[inline]
    fn get_ids(inp: &str) -> impl Iterator<Item = usize> + '_ {
        inp
            .split(',')
            .flat_map(|s| {
                let (a, b) = s.split_once('-').unwrap();
                a.parse().unwrap()..=b.parse().unwrap()
            })
    }
}

impl Solution for Day2 {
    const NAME: &'static str = "Gift Shop";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        Self::get_ids(inp)
            .filter(|id| {
                let str_id = id.to_string();
                let (a, b) = str_id.split_at(str_id.len() / 2);

                a == b
            })
            .sum()
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        Self::get_ids(inp)
            .filter(|id|
                PAT.is_match(&id.to_string())
                    .unwrap_or(false)
            )
            .sum()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 43_952_536_386);
        assert_eq!(p2, 54_486_209_192);
    }
}

fn main() {
    aoc_2025::run_day(2, &Day2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}