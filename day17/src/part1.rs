use arrayvec::ArrayVec;
use grid::Grid;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn solve(input: &str) -> Option<usize> {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let cols = lines[0].len();
    let grid = Grid::from_vec(
        lines.into_iter().flatten().map(|c| *c - b'0').collect(),
        cols,
    );
    let target_location = (grid.rows() - 1, grid.cols() - 1);

    let mut queue = start_states(&grid);
    let mut visited = HashSet::new();

    while let Some(Reverse(state)) = queue.pop() {
        if state.position == target_location {
            return Some(state.cost);
        } else {
            let vertical = state.direction == Direction::Up || state.direction == Direction::Down;
            if visited.insert((state.position, vertical)) {
                queue.extend(state.next_states(&grid));
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
}

fn start_states(grid: &Grid<u8>) -> BinaryHeap<Reverse<State>> {
    let mut states = BinaryHeap::new();
    let mut right_cost = 0;
    let mut down_cost = 0;
    for n in 1..=3 {
        right_cost += grid[(0, n)] as usize;
        states.push(Reverse(State {
            cost: right_cost,
            position: (0, n),
            direction: Direction::Right,
        }));
        down_cost += grid[(n, 0)] as usize;
        states.push(Reverse(State {
            cost: down_cost,
            position: (n, 0),
            direction: Direction::Down,
        }))
    }
    states
}

impl State {
    fn next_states(&self, grid: &Grid<u8>) -> ArrayVec<Reverse<State>, 6> {
        let mut states = ArrayVec::new();
        match self.direction {
            Direction::Down | Direction::Up => {
                let mut left_cost = self.cost;
                let mut right_cost = self.cost;
                for n in 1..=3 {
                    if self.position.1 >= n {
                        let position = (self.position.0, self.position.1 - n);
                        left_cost += grid[position] as usize;
                        states.push(Reverse(State {
                            cost: left_cost,
                            position,
                            direction: Direction::Left,
                        }))
                    }
                    if self.position.1 + n < grid.cols() {
                        let position = (self.position.0, self.position.1 + n);
                        right_cost += grid[position] as usize;
                        states.push(Reverse(State {
                            cost: right_cost,
                            position,
                            direction: Direction::Right,
                        }))
                    }
                }
            }
            Direction::Left | Direction::Right => {
                let mut up_cost = self.cost;
                let mut down_cost = self.cost;
                for n in 1..=3 {
                    if self.position.0 >= n {
                        let position = (self.position.0 - n, self.position.1);
                        up_cost += grid[position] as usize;
                        states.push(Reverse(State {
                            cost: up_cost,
                            position,
                            direction: Direction::Up,
                        }))
                    }
                    if self.position.0 + n < grid.rows() {
                        let position = (self.position.0 + n, self.position.1);
                        down_cost += grid[position] as usize;
                        states.push(Reverse(State {
                            cost: down_cost,
                            position,
                            direction: Direction::Down,
                        }))
                    }
                }
            }
        }
        states
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, Some(859));
    }
}
