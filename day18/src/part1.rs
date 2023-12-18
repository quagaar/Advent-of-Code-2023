use itertools::Itertools;
use std::ops::Range;

pub fn solve(input: &str) -> u32 {
    let trenches = input
        .lines()
        .scan(Position { x: 0, y: 0 }, Trench::create)
        .collect::<Vec<_>>();

    let (min_y, max_y) = trenches
        .iter()
        .fold((0, 0), |(min_y, max_y), trench| match trench {
            Trench::Up(_, y_range) => (min_y.min(y_range.start), max_y.max(y_range.end - 1)),
            Trench::Down(_, y_range) => (min_y.min(y_range.start), max_y.max(y_range.end - 1)),
            Trench::Left(y, _) => (min_y.min(*y), max_y.max(*y)),
            Trench::Right(y, _) => (min_y.min(*y), max_y.max(*y)),
        });

    (min_y..=max_y)
        .map(|y| {
            trenches
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
                .0
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Trench {
    Up(i32, Range<i32>),
    Down(i32, Range<i32>),
    Left(i32, Range<i32>),
    Right(i32, Range<i32>),
}

impl Trench {
    fn create(position: &mut Position, line: &str) -> Option<Self> {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[2..].split_once(' ').unwrap().0.parse().unwrap();
        match direction {
            'U' => {
                let y_range = position.y + 1..position.y + distance + 1;
                position.y += distance;
                Some(Self::Up(position.x, y_range))
            }
            'D' => {
                let y_range = position.y - distance..position.y;
                position.y -= distance;
                Some(Self::Down(position.x, y_range))
            }
            'L' => {
                let x_range = position.x - distance..position.x;
                position.x -= distance;
                Some(Self::Left(position.y, x_range))
            }
            'R' => {
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
        assert_eq!(result, 62);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 34329);
    }
}
