//! Day 4: Printing Department
//!
//! <https://adventofcode.com/2025/day/4>
use std::fmt::Display;
use aoc_2025::{Solution, get_grid, neighbors};

pub struct Day4;

impl Solution for Day4 {
    const NAME: &'static str = "Printing Department";

    fn part_one<T: Display>(&self, inp: T) -> Self::OutputP1 {
        let grid = get_grid(inp);

        let n_rows = 0..grid.len();
        let n_cols  = 0..grid[0].len();

        grid
            .iter()
            .enumerate()
            .map(|(i, row)| row
                .iter()
                .enumerate()
                .filter(|&(j, &cell)| cell == b'@' && neighbors(i, j)
                    .iter()
                    .filter(|(i, j)| n_rows.contains(i) && n_cols.contains(j) && grid[*i][*j] == b'@')
                    .count() < 4
                )
                .count()
            )
            .sum()
    }

    fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
        let mut grid = get_grid(inp);

        let n_rows = grid.len();
        let n_cols  = grid[0].len();

        let mut count = 0;

        loop {
            let mut accessible = 0;

            for i in 0..n_rows {
                for j in 0..n_cols {
                    if grid[i][j] == b'@' && neighbors(i, j)
                        .iter()
                        .filter(|(i, j)|
                            (0..n_rows).contains(i)
                            && (0..n_cols).contains(j)
                            && grid[*i][*j] == b'@'
                        )
                        .count() < 4
                    {
                        grid[i][j] = b'.';
                        accessible += 1;
                    }
                }
            }

            if accessible == 0 {
                break;
            }
            count += accessible;
        }
        count
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1460);
        assert_eq!(p2, 9243);
    }
}

fn main() {
    aoc_2025::run_day(4, &Day4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}