use arrayvec::ArrayVec;
use grid::Grid;
use itertools::Itertools;
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

    let mut beams = VecDeque::from([Beam {
        direction: Direction::Right,
        position: (0, 0),
    }]);
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 6816);
    }
}
