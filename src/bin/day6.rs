//! Day 6: Trash Compactor
//!
//! <https://adventofcode.com/2025/day/6>
use aoc_2025::{get_grid, Solution};

pub struct Day6;

impl Solution for Day6 {
    const NAME: &'static str = "Trash Compactor";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        let grid = inp
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        (0..n_cols)
            .map(|j| {
                let iter = (0..n_rows - 1).map(|i| grid[i][j].parse::<usize>().unwrap());

                if grid.last().unwrap()[j] == "*" {
                    iter.product::<usize>()
                } else {
                    iter.sum()
                }
            })
            .sum()
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        let grid = get_grid(inp);

        let n_rows = grid.len();
        let n_cols = grid.iter().map(Vec::len).max().unwrap();

        let mut total = 0;
        let mut curr = 0;

        let mut op = 0;

        for j in 0..n_cols {
            let last = grid.last().unwrap();

            if j < last.len() && last[j] != b' ' {
                op = last[j];
                total += curr;
                curr = usize::from(op == b'*');
            }

            let mut num = String::new();
            for row in grid.iter().take(n_rows - 1) {
                num.push(if j < row.len() { row[j] } else { b' ' } as char);
            }

            if let Ok(num) = num.trim().parse::<usize>() {
                if op == b'*' {
                    curr *= num;
                } else {
                    curr += num;
                }
            }
        }
        total + curr
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 3_785_892_992_137);
        assert_eq!(p2, 7_669_802_156_452);
    }
}

fn main() {
    aoc_2025::run_day(6, &Day6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
}
