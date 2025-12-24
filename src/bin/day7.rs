//! Day 7: Laboratories
//!
//! <https://adventofcode.com/2025/day/7>
use std::fmt::Display;
use aoc_2025::{Solution, get_grid};

pub struct Day7;

impl Solution for Day7 {
    const NAME: &'static str = "Laboratories";

    fn part_one<T: Display>(&self, inp: T) -> Self::OutputP1 {
        let mut grid = get_grid(inp);

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        let mut split = 0;

        for i in 1..n_rows {
            for j in 0..n_cols {
                if grid[i - 1][j] == b'S' || grid[i - 1][j] == b'|' {
                    let row = &mut grid[i];

                    if row[j] == b'.' {
                        row[j] = b'|';
                    } else if row[j] == b'^' {
                        split += 1;

                        if j > 0 {
                            row[j - 1] = b'|';
                        }
                        if j < n_cols {
                            row[j + 1] = b'|';
                        }
                    }
                }
            }
        }
        split
    }

    fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
        let mut grid = inp
            .to_string()
            .lines()
            .map(|line| {
                line
                    .bytes()
                    .map(|cell| match cell {
                        b'.' => 0isize,
                        b'S' => 1,
                        _ => -1,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        for i in 1..n_rows {
            for j in 0..n_cols {
                let prev = grid[i - 1][j];
                let row = &mut grid[i];

                if prev > 0 {
                    if row[j] == -1 {
                        if j > 0 {
                            row[j - 1] += prev;
                        }
                        if j < n_cols {
                            row[j + 1] += prev;
                        }
                    } else {
                        row[j] += prev;
                    }
                }
            }
        }
        grid
            .last()
            .unwrap()
            .iter()
            .sum::<isize>()
            .cast_unsigned()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1630);
        assert_eq!(p2, 47_857_642_990_160);
    }
}

fn main() {
    aoc_2025::run_day(7, &Day7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}