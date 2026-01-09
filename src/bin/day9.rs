//! Day 9: Movie Theater
//!
//! <https://adventofcode.com/2025/day/9>
use aoc_2025::Solution;
use itertools::Itertools;

type Point = (usize, usize);

pub struct Day9;

impl Day9 {
    #[must_use]
    #[inline]
    fn parse_input(inp: &str) -> Vec<Point> {
        inp.lines()
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();

                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect()
    }

    #[must_use]
    fn point_in_polygon(x: usize, y: usize, polygon: &[Point]) -> bool {
        let mut inside = false;
        let n_points = polygon.len();

        for i in 0..n_points {
            let (ax, ay) = polygon[i];

            let (bx, by) = if i == n_points - 1 {
                polygon[0]
            } else {
                polygon[i + 1]
            };

            if ax == bx && (ay > y) != (by > y) && x < ax {
                inside = !inside;
            }
        }
        inside
    }

    #[must_use]
    fn intersects(
        line1_start: Point,
        line1_end: Point,
        line2_start: Point,
        line2_end: Point,
    ) -> bool {
        if (line1_start.0 == line1_end.0) == (line2_start.0 == line2_end.0) {
            return false;
        }

        let start1 = line1_start.min(line1_end);
        let end1 = line1_start.max(line1_end);
        let start2 = line2_start.min(line2_end);
        let end2 = line2_start.max(line2_end);

        if start1.0 == end1.0 {
            start2.0 < start1.0 && start1.0 < end2.0 && start1.1 < start2.1 && start2.1 < end1.1
        } else {
            start1.0 < start2.0 && start2.0 < end1.0 && start2.1 < start1.1 && start1.1 < end2.1
        }
    }
}

impl Solution for Day9 {
    const NAME: &'static str = "Movie Theater";

    fn part_one(&self, inp: &str) -> Self::OutputP1 {
        Self::parse_input(inp)
            .iter()
            .tuple_combinations()
            .map(|(&(x1, y1), &(x2, y2))| (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1))
            .max()
            .unwrap()
    }

    fn part_two(&self, inp: &str) -> Self::OutputP2 {
        let points = Self::parse_input(inp);
        let mut max_area = 0;

        let n_points = points.len();

        'outer: for (&(x1, y1), &(x2, y2)) in points.iter().tuple_combinations() {
            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);

            if area <= max_area {
                continue;
            }

            let rect_corners = [
                (x1.min(x2) + 1, y1.min(y2) + 1),
                (x1.min(x2) + 1, y1.max(y2) - 1),
                (x1.max(x2) - 1, y1.max(y2) - 1),
                (x1.max(x2) - 1, y1.min(y2) + 1),
            ];

            if !rect_corners
                .iter()
                .all(|&(px, py)| Self::point_in_polygon(px, py, &points))
            {
                continue;
            }

            for i in 0..4 {
                let edge_start = rect_corners[i];

                let edge_end = if i == 3 {
                    rect_corners[0]
                } else {
                    rect_corners[i + 1]
                };

                for j in 0..n_points {
                    let line_start = points[j];

                    let line_end = if j == n_points - 1 {
                        points[0]
                    } else {
                        points[j + 1]
                    };

                    if Self::intersects(edge_start, edge_end, line_start, line_end) {
                        continue 'outer;
                    }
                }
            }

            max_area = area;
        }
        max_area
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 4_777_967_538);
        assert_eq!(p2, 1_439_894_345);
    }
}

fn main() {
    aoc_2025::run_day(9, &Day9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        main();
    }
}
