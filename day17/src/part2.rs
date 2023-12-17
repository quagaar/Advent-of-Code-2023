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

const MIN_STEPS: usize = 4;
const MAX_STEPS: usize = 10;
const NEXT_STATES_MAX: usize = (MAX_STEPS - MIN_STEPS + 1) * 2;

fn start_states(grid: &Grid<u8>) -> BinaryHeap<Reverse<State>> {
    let mut states = BinaryHeap::new();
    let mut right_cost = (1..MIN_STEPS)
        .filter_map(|n| grid.get(0, n))
        .map(|n| *n as usize)
        .sum();
    let mut down_cost = (1..MIN_STEPS)
        .filter_map(|n| grid.get(n, 0))
        .map(|n| *n as usize)
        .sum();
    for n in MIN_STEPS..=MAX_STEPS {
        if let Some(cost) = grid.get(0, n) {
            right_cost += *cost as usize;
            states.push(Reverse(State {
                cost: right_cost,
                position: (0, n),
                direction: Direction::Right,
            }));
        }
        if let Some(cost) = grid.get(n, 0) {
            down_cost += *cost as usize;
            states.push(Reverse(State {
                cost: down_cost,
                position: (n, 0),
                direction: Direction::Down,
            }))
        }
    }
    states
}

impl State {
    fn next_states(&self, grid: &Grid<u8>) -> ArrayVec<Reverse<State>, NEXT_STATES_MAX> {
        let mut states = ArrayVec::new();
        match self.direction {
            Direction::Down | Direction::Up => {
                let mut left_cost = self.left_cost(MIN_STEPS - 1, grid);
                let mut right_cost = self.right_cost(MIN_STEPS - 1, grid);
                for n in MIN_STEPS..=MAX_STEPS {
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
                let mut up_cost = self.up_cost(MIN_STEPS - 1, grid);
                let mut down_cost = self.down_cost(MIN_STEPS - 1, grid);
                for n in MIN_STEPS..=MAX_STEPS {
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

    fn right_cost(&self, steps: usize, grid: &Grid<u8>) -> usize {
        self.cost
            + (1..=steps)
                .filter_map(|n| grid.get(self.position.0, self.position.1 + n))
                .map(|n| *n as usize)
                .sum::<usize>()
    }

    fn left_cost(&self, steps: usize, grid: &Grid<u8>) -> usize {
        self.cost
            + (1..=steps)
                .filter(|n| self.position.1 >= *n)
                .map(|n| grid[(self.position.0, self.position.1 - n)] as usize)
                .sum::<usize>()
    }

    fn up_cost(&self, steps: usize, grid: &Grid<u8>) -> usize {
        self.cost
            + (1..=steps)
                .filter(|n| self.position.0 >= *n)
                .map(|n| grid[(self.position.0 - n, self.position.1)] as usize)
                .sum::<usize>()
    }

    fn down_cost(&self, steps: usize, grid: &Grid<u8>) -> usize {
        self.cost
            + (1..=steps)
                .filter_map(|n| grid.get(self.position.0 + n, self.position.1))
                .map(|n| *n as usize)
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, Some(94));
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, Some(71));
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, Some(1027));
    }
}
