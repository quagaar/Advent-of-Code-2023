use arrayvec::ArrayVec;
use grid::Grid;
use pathfinding::directed::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Node {
    Start(usize, usize),
    Horizontal((usize, usize)),
    Vertical((usize, usize)),
}

const MAX_STEPS: usize = 3;

pub fn solve(input: &str) -> Option<usize> {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let cols = lines[0].len();
    let grid = Grid::from_vec(
        lines.into_iter().flatten().map(|c| *c - b'0').collect(),
        cols,
    );
    let target_location = (grid.rows() - 1, grid.cols() - 1);

    dijkstra::dijkstra(
        &Node::Start(0, 0),
        |&node| {
            let mut states: ArrayVec<(Node, usize), 6> = ArrayVec::new();
            match node {
                Node::Start(row, column) => {
                    let mut right_cost = 0;
                    let mut down_cost = 0;
                    for n in 1..=MAX_STEPS {
                        if let Some(cost) = grid.get(row, column + n) {
                            right_cost += *cost as usize;
                            states.push((Node::Horizontal((row, column + n)), right_cost));
                        }
                        if let Some(cost) = grid.get(row + n, column) {
                            down_cost += *cost as usize;
                            states.push((Node::Vertical((row + n, column)), down_cost));
                        }
                    }
                }
                Node::Horizontal((row, column)) => {
                    let mut up_cost = 0;
                    let mut down_cost = 0;
                    for n in 1..=MAX_STEPS {
                        if row >= n {
                            let position = (row - n, column);
                            up_cost += grid[position] as usize;
                            states.push((Node::Vertical(position), up_cost))
                        }
                        if row + n < grid.rows() {
                            let position = (row + n, column);
                            down_cost += grid[position] as usize;
                            states.push((Node::Vertical(position), down_cost))
                        }
                    }
                }
                Node::Vertical((row, column)) => {
                    let mut left_cost = 0;
                    let mut right_cost = 0;
                    for n in 1..=MAX_STEPS {
                        if column >= n {
                            let position = (row, column - n);
                            left_cost += grid[position] as usize;
                            states.push((Node::Horizontal(position), left_cost))
                        }
                        if column + n < grid.cols() {
                            let position = (row, column + n);
                            right_cost += grid[position] as usize;
                            states.push((Node::Horizontal(position), right_cost))
                        }
                    }
                }
            }
            states
        },
        |&node| {
            node == Node::Horizontal(target_location) || node == Node::Vertical(target_location)
        },
    )
    .map(|(_, cost)| cost)
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
