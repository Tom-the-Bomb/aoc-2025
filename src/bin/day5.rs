//! Day 5: Cafeteria
//!
//! <https://adventofcode.com/2025/day/5>
use aoc_2025::Solution;

pub struct Day5;

impl Solution for Day5 {
    const NAME: &'static str = "Cafeteria";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        let inp = inp.replace('\r', "");

        let (ranges, ingredients) = inp
            .split_once("\n\n")
            .unwrap();

        let ranges = ranges
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();

                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect::<Vec<(usize, usize)>>();

        ingredients
            .lines()
            .filter(|ingredient| ranges
                .iter()
                .any(|&(a, b)| {
                    let ingredient = ingredient.parse().unwrap();
                    a <= ingredient && ingredient <= b
                })
            )
            .count()
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        let inp = inp.replace('\r', "");

        let mut ranges = inp
            .split_once("\n\n")
            .unwrap()
            .0
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();

                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect::<Vec<(usize, usize)>>();

        ranges.sort_unstable();

        let mut i = 1;
        while i < ranges.len() {
            let (a0, b0) = ranges[i - 1];
            let (a, b) = ranges[i];

            if a0 <= a && a <= b0 {
                ranges[i - 1] = (a0, b.max(b0));
                ranges.remove(i);
            } else {
                i += 1;
            }
        }

        ranges
            .into_iter()
            .map(|(a, b)| b - a + 1)
            .sum()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 874);
        assert_eq!(p2, 348_548_952_146_313);
    }
}

fn main() {
    aoc_2025::run_day(5, &Day5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}