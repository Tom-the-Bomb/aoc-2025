//! Day 10: Factory
//!
//! <https://adventofcode.com/2025/day/10>
#![feature(strip_circumfix)]

use std::fmt::Display;
use itertools::Itertools;
use good_lp::{
    variables,
    variable,
    Expression,
    default_solver,
    SolverModel,
    Solution as _
};
use aoc_2025::Solution;

pub struct Day10;

impl Day10 {
    #[inline]
    fn parse_input(inp: &str) -> impl Iterator<Item = (&[u8], Vec<Vec<usize>>, Vec<u32>)> {
        inp
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();

                let lights = parts
                    .next()
                    .unwrap()
                    .strip_circumfix('[', ']')
                    .unwrap()
                    .as_bytes();

                let joltages = parts
                    .next_back()
                    .unwrap()
                    .strip_circumfix('{', '}')
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect();

                let schematics = parts
                    .map(|schematic| {
                        schematic
                            .strip_circumfix('(', ')')
                            .unwrap()
                            .split(',')
                            .map(|x| x.parse().unwrap())
                            .collect()
                    })
                    .collect();

                (lights, schematics, joltages)
            })
    }
}

impl Solution for Day10 {
    const NAME: &'static str = "Factory";

    fn part_one<T: Display>(&self, inp: T) -> Self::OutputP1 {
        let mut total = 0;

        for (target, schematics, _) in Self::parse_input(&inp.to_string()) {
            'outer: for n in 1.. {
                for combo in schematics
                    .iter()
                    .combinations_with_replacement(n)
                {
                    let mut lights = vec![b'.'; target.len()];

                    for schematic in combo {
                        for &index in schematic {
                            lights[index] = match lights[index] {
                                b'.' => b'#',
                                _ => b'.',
                            }
                        }
                    }

                    if &*lights == target {
                        total += n;
                        break 'outer;
                    }
                }
            }
        }
        total
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
        let mut total = 0.0;

        for (_, schematics, target) in Self::parse_input(&inp.to_string()) {
            let mut problem = variables!();
            let vars: Vec<good_lp::Variable> = problem.add_all(
                (0..schematics.len())
                    .map(|_| variable().integer().min(0))
            );
            let objective = vars.iter().sum::<Expression>();

            total += problem
                .minimise(&objective)
                .using(default_solver)
                .with_all(
                    target
                        .iter()
                        .enumerate()
                        .map(|(i, &joltage)|
                            schematics
                                .iter()
                                .enumerate()
                                .filter_map(|(s, schematic)|
                                    schematic.contains(&i)
                                        .then_some(&vars[s])
                                )
                                .sum::<Expression>()
                                .eq(joltage)
                        )
                )
                .solve()
                .unwrap()
                .eval(&objective);
        }
        total as usize
    }

    // Z3 approach (doesn't work)
    //
    // fn part_two<T: Display>(&self, inp: T) -> Self::OutputP2 {
    //     let mut total = 0;

    //     for (_, schematics, target) in Self::parse_input(&inp.to_string()) {
    //         let opt = Optimize::new();

    //         let vars = (0..schematics.len())
    //             .map(|i| Int::new_const(format!("n{i}")))
    //             .collect::<Vec<_>>();

    //         for var in &vars {
    //             opt.assert(&var.ge(0));
    //         }

    //         for (i, &joltage) in target.iter().enumerate() {
    //             opt.assert(
    //                 &schematics
    //                     .iter()
    //                     .enumerate()
    //                     .filter_map(|(s, schematic)|
    //                         schematic.contains(&i)
    //                             .then_some(&vars[s])
    //                     )
    //                     .sum::<Int>()
    //                     .eq(joltage)
    //             )
    //         }

    //         let var_sum = vars.iter().sum::<Int>();

    //         opt.minimize(&var_sum);
    //         opt.check(&[]);

    //         total += opt
    //             .get_model()
    //             .unwrap()
    //             .eval(&var_sum, true)
    //             .unwrap()
    //             .as_u64()
    //             .unwrap();
    //     }
    //     total
    // }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 481);
        assert_eq!(p2, 20_142);
    }
}

fn main() {
    aoc_2025::run_day(10, &Day10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}