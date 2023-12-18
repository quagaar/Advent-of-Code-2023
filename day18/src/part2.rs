use itertools::Itertools;
use std::{collections::BTreeSet, ops::Range};

pub fn solve(input: &str) -> u64 {
    let trenches = input
        .lines()
        .scan(Position { x: 0, y: 0 }, Trench::create)
        .collect::<Vec<_>>();

    let y_ranges: BTreeSet<i64> = trenches
        .iter()
        .flat_map(|trench| match trench {
            Trench::Up(_, y_range) => [y_range.start, y_range.end],
            Trench::Down(_, y_range) => [y_range.start, y_range.end],
            Trench::Left(y, _) => [*y, *y + 1],
            Trench::Right(y, _) => [*y, *y + 1],
        })
        .collect();

    y_ranges
        .into_iter()
        .tuple_windows()
        .map(|(y, next)| {
            let repeat = (next - y) as u64;
            let total = trenches
                .iter()
                .enumerate()
                .filter(|(_, trench)| match trench {
                    Trench::Up(_, y_range) => y_range.contains(&y),
                    Trench::Down(_, y_range) => y_range.contains(&y),
                    Trench::Left(trench_y, _) => y == *trench_y,
                    Trench::Right(trench_y, _) => y == *trench_y,
                })
                .sorted_by_key(|(_, trench)| match trench {
                    Trench::Up(x, _) => *x,
                    Trench::Down(x, _) => *x,
                    Trench::Left(_, x_range) => x_range.start,
                    Trench::Right(_, x_range) => x_range.start,
                })
                .fold((0, None), |(total, start_pos), (n, trench)| match trench {
                    Trench::Up(x, _) => {
                        let start = start_pos.unwrap_or(*x);
                        (total, Some(start))
                    }
                    Trench::Down(x, _) => {
                        let start = start_pos.expect("No start position");
                        let total = total + start.abs_diff(*x) + 1;
                        (total, None)
                    }
                    Trench::Left(_, x_range) => {
                        let start = start_pos.unwrap_or(x_range.start);
                        (total, Some(start))
                    }
                    Trench::Right(_, x_range) => {
                        let start = start_pos.unwrap_or(x_range.start);
                        let next = trenches.get(n + 1).unwrap_or(&trenches[0]);
                        match next {
                            Trench::Up(_, _) => (total, Some(start)),
                            Trench::Down(_, _) => {
                                let total = total + start.abs_diff(x_range.end);
                                (total, None)
                            }
                            _ => panic!("Unexpected trench after right: {:?}", next),
                        }
                    }
                })
                .0;
            total * repeat
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Trench {
    Up(i64, Range<i64>),
    Down(i64, Range<i64>),
    Left(i64, Range<i64>),
    Right(i64, Range<i64>),
}

impl Trench {
    fn create(position: &mut Position, line: &str) -> Option<Self> {
        let (_, hex) = line.split_once(" (#").unwrap();
        let distance = i64::from_str_radix(&hex[0..5], 16).unwrap();
        let direction = hex.chars().nth(5).unwrap();
        match direction {
            '3' => {
                let y_range = position.y + 1..position.y + distance + 1;
                position.y += distance;
                Some(Self::Up(position.x, y_range))
            }
            '1' => {
                let y_range = position.y - distance..position.y;
                position.y -= distance;
                Some(Self::Down(position.x, y_range))
            }
            '2' => {
                let x_range = position.x - distance..position.x;
                position.x -= distance;
                Some(Self::Left(position.y, x_range))
            }
            '0' => {
                let x_range = position.x + 1..position.x + distance + 1;
                position.x += distance;
                Some(Self::Right(position.y, x_range))
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 42617947302920);
    }
}
