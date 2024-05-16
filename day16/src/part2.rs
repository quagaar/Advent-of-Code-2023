use arrayvec::ArrayVec;
use grid::Grid;
use itertools::{chain, Itertools};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let grid = Grid::from_vec(
        lines
            .into_iter()
            .flatten()
            .copied()
            .map(|c| Tile::try_from(c).unwrap())
            .collect(),
        width,
    );

    chain![
        (0..grid.rows()).map(|row| Beam {
            direction: Direction::Right,
            position: (row, 0),
        }),
        (0..grid.rows()).map(|row| Beam {
            direction: Direction::Left,
            position: (row, grid.cols() - 1),
        }),
        (0..grid.cols()).map(|col| Beam {
            direction: Direction::Down,
            position: (0, col),
        }),
        (0..grid.cols()).map(|col| Beam {
            direction: Direction::Up,
            position: (grid.rows() - 1, col),
        }),
    ]
    .par_bridge()
    .map(|start| get_energized_count(start, &grid))
    .max()
    .unwrap()
}

fn get_energized_count(start: Beam, grid: &Grid<Tile>) -> usize {
    let mut beams = VecDeque::from([start]);
    let mut history = HashSet::new();

    while let Some(beam) = beams.pop_front() {
        if history.contains(&beam) {
            continue;
        } else {
            history.insert(beam);
        }
        match grid[beam.position] {
            Tile::HSplitter => beam
                .split_horizontal()
                .into_iter()
                .filter_map(|beam| beam.next(grid.rows(), grid.cols()))
                .for_each(|b| beams.push_back(b)),
            Tile::VSplitter => beam
                .split_vertical()
                .into_iter()
                .filter_map(|beam| beam.next(grid.rows(), grid.cols()))
                .for_each(|b| beams.push_back(b)),
            Tile::NEMirror => {
                if let Some(beam) = beam.ne_mirror().next(grid.rows(), grid.cols()) {
                    beams.push_back(beam);
                }
            }
            Tile::NWMirror => {
                if let Some(beam) = beam.nw_mirror().next(grid.rows(), grid.cols()) {
                    beams.push_back(beam);
                }
            }
            Tile::Empty => {
                if let Some(beam) = beam.next(grid.rows(), grid.cols()) {
                    beams.push_back(beam);
                }
            }
        }
    }

    history.into_iter().unique_by(|beam| beam.position).count()
}

enum Tile {
    Empty,
    NEMirror,
    NWMirror,
    HSplitter,
    VSplitter,
}

impl TryFrom<u8> for Tile {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Tile::Empty),
            b'/' => Ok(Tile::NEMirror),
            b'\\' => Ok(Tile::NWMirror),
            b'-' => Ok(Tile::HSplitter),
            b'|' => Ok(Tile::VSplitter),
            _ => Err(format!("Invalid tile type: {}", value as char)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    direction: Direction,
    position: (usize, usize),
}

impl Beam {
    fn next(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.direction {
            Direction::Right => {
                if (self.position.1 + 1) < cols {
                    self.position.1 += 1;
                    Some(self)
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.position.1 > 0 {
                    self.position.1 -= 1;
                    Some(self)
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.position.0 > 0 {
                    self.position.0 -= 1;
                    Some(self)
                } else {
                    None
                }
            }
            Direction::Down => {
                if (self.position.0 + 1) < rows {
                    self.position.0 += 1;
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    fn split_horizontal(mut self) -> ArrayVec<Self, 2> {
        match self.direction {
            Direction::Up | Direction::Down => {
                self.direction = Direction::Left;
                ArrayVec::from([
                    Self {
                        direction: Direction::Right,
                        position: self.position,
                    },
                    self,
                ])
            }
            _ => ArrayVec::from_iter([self]),
        }
    }

    fn split_vertical(mut self) -> ArrayVec<Self, 2> {
        match self.direction {
            Direction::Right | Direction::Left => {
                self.direction = Direction::Down;
                ArrayVec::from([
                    Self {
                        direction: Direction::Up,
                        position: self.position,
                    },
                    self,
                ])
            }
            _ => ArrayVec::from_iter([self]),
        }
    }

    fn nw_mirror(mut self) -> Self {
        match self.direction {
            Direction::Right => self.direction = Direction::Down,
            Direction::Left => self.direction = Direction::Up,
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
        }
        self
    }

    fn ne_mirror(mut self) -> Self {
        match self.direction {
            Direction::Right => self.direction = Direction::Up,
            Direction::Left => self.direction = Direction::Down,
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 51);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
