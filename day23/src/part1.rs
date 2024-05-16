use grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let cols = lines[0].len();
    let grid = Grid::from_vec(lines.into_iter().flatten().copied().collect(), cols);
    let start = Node { row: 0, col: 1 };
    let target = Node {
        row: grid.rows() - 1,
        col: grid.cols() - 2,
    };

    let graph = build_graph(&grid, start, target);

    #[cfg(debug_assertions)]
    print_graph(&graph, grid.cols());

    let mut visited = vec![NodeId::from(start, grid.cols())];

    longest_path(
        NodeId::from(start, grid.cols()),
        0,
        &graph,
        NodeId::from(target, grid.cols()),
        &mut visited,
    )
    .unwrap()
}

#[allow(dead_code)]
fn print_graph(graph: &Graph, cols: usize) {
    println!("digraph {{");
    for (node, edges) in graph {
        let node_idx = graph.iter().position(|(n, _)| n == node).unwrap();
        let row = node.row(cols);
        let col = node.col(cols);
        if row == 0 && col == 1 {
            println!("   node{node_idx} [label=\"start\" style=filled]");
        } else {
            println!("   node{node_idx} [label=\"{row},{col}\"]");
        }

        for edge in edges {
            let length = edge.length;
            if let Some(target_idx) = graph.iter().position(|(n, _)| n == &edge.to) {
                println!("   node{node_idx} -> node{target_idx} [label={length}]");
            } else {
                println!("   node{node_idx} -> target [label={length}]");
            }
        }
    }
    println!("   target [style=filled]");
    println!("}}");
}

fn longest_path(
    node: NodeId,
    distance: usize,
    graph: &Graph,
    target: NodeId,
    visited: &mut Vec<NodeId>,
) -> Option<usize> {
    graph.get(&node).and_then(|edges| {
        edges
            .iter()
            .filter_map(|edge| {
                if edge.to == target {
                    Some(distance + edge.length)
                } else if visited.contains(&edge.to) {
                    None
                } else {
                    visited.push(edge.to);
                    let result =
                        longest_path(edge.to, distance + edge.length, graph, target, visited);
                    visited.pop();
                    result
                }
            })
            .max()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(u16);

impl NodeId {
    fn from(node: Node, cols: usize) -> Self {
        Self((node.row * cols + node.col) as u16)
    }

    fn row(&self, cols: usize) -> usize {
        self.0 as usize / cols
    }

    fn col(&self, cols: usize) -> usize {
        self.0 as usize % cols
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    length: usize,
    to: NodeId,
}

type Graph = HashMap<NodeId, Vec<Edge>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

fn build_graph(grid: &Grid<u8>, start: Node, target: Node) -> Graph {
    let mut graph = Graph::new();
    let mut queue = VecDeque::from([(start, Direction::Down)]);
    let mut visited = HashSet::new();

    while let Some((node, mut direction)) = queue.pop_front() {
        if visited.insert((node, direction)) {
            let mut position = node.step(direction);
            for length in 1.. {
                match next_positions(position, direction, grid) {
                    [Some(up), None, None, None] => {
                        direction = Direction::Up;
                        position = up;
                    }
                    [None, Some(down), None, None] => {
                        direction = Direction::Down;
                        position = down;
                    }
                    [None, None, Some(left), None] => {
                        direction = Direction::Left;
                        position = left;
                    }
                    [None, None, None, Some(right)] => {
                        direction = Direction::Right;
                        position = right;
                    }
                    [None, None, None, None] if position != target => break,
                    next => {
                        graph
                            .entry(NodeId::from(node, grid.cols()))
                            .or_default()
                            .push(Edge {
                                length,
                                to: NodeId::from(position, grid.cols()),
                            });
                        if position != target {
                            if let Some(_up) = next[0] {
                                queue.push_back((position, Direction::Up));
                            }
                            if let Some(_down) = next[1] {
                                queue.push_back((position, Direction::Down));
                            }
                            if let Some(_left) = next[2] {
                                queue.push_back((position, Direction::Left));
                            }
                            if let Some(_right) = next[3] {
                                queue.push_back((position, Direction::Right));
                            }
                            queue.push_back((position, direction.opposite()));
                        }
                        break;
                    }
                }
            }
        }
    }

    graph
}

fn next_positions(position: Node, direction: Direction, grid: &Grid<u8>) -> [Option<Node>; 4] {
    match direction {
        Direction::Up => [
            position.try_step_up(grid),
            None,
            position.try_step_left(grid),
            position.try_step_right(grid),
        ],
        Direction::Down => [
            None,
            position.try_step_down(grid),
            position.try_step_left(grid),
            position.try_step_right(grid),
        ],
        Direction::Left => [
            position.try_step_up(grid),
            position.try_step_down(grid),
            position.try_step_left(grid),
            None,
        ],
        Direction::Right => [
            position.try_step_up(grid),
            position.try_step_down(grid),
            None,
            position.try_step_right(grid),
        ],
    }
}

impl Node {
    fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                row: self.row.saturating_sub(1),
                col: self.col,
            },
            Direction::Down => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Self {
                row: self.row,
                col: self.col.saturating_sub(1),
            },
            Direction::Right => Self {
                row: self.row,
                col: self.col + 1,
            },
        }
    }

    fn try_step_up(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.row == 0 {
            None
        } else {
            match grid.get(self.row - 1, self.col) {
                Some(b'.') | Some(b'^') => Some(Self {
                    row: self.row - 1,
                    col: self.col,
                }),
                _ => None,
            }
        }
    }

    fn try_step_down(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.row == grid.rows() - 1 {
            None
        } else {
            match grid.get(self.row + 1, self.col) {
                Some(b'.') | Some(b'v') => Some(Self {
                    row: self.row + 1,
                    col: self.col,
                }),
                _ => None,
            }
        }
    }

    fn try_step_left(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.col == 0 {
            None
        } else {
            match grid.get(self.row, self.col - 1) {
                Some(b'.') | Some(b'<') => Some(Self {
                    row: self.row,
                    col: self.col - 1,
                }),
                _ => None,
            }
        }
    }

    fn try_step_right(&self, grid: &Grid<u8>) -> Option<Self> {
        if self.col == grid.cols() - 1 {
            None
        } else {
            match grid.get(self.row, self.col + 1) {
                Some(b'.') | Some(b'>') => Some(Self {
                    row: self.row,
                    col: self.col + 1,
                }),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 94);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
