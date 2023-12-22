use rayon::prelude::*;
use std::{collections::HashSet, ops::Range};

pub fn solve(input: &str) -> usize {
    let mut stack = input
        .lines()
        .enumerate()
        .map(|(n, line)| Brick::parse(line, n))
        .collect::<Vec<_>>();

    settle_stack(&mut stack);

    let graph = stack
        .iter()
        .map(|brick| (brick.id, brick.get_supporters(&stack)))
        .collect::<Vec<_>>();

    stack
        .par_iter()
        .map(|brick| brick.chain_reaction_count(&graph))
        .sum()
}

/// Settles the stack by dropping bricks to the lowest possible position.
fn settle_stack(stack: &mut Vec<Brick>) {
    stack.sort_by_key(|brick| brick.z.start);
    for n in 0..stack.len() {
        if let Some(drop) = stack
            .iter()
            .filter_map(|other| stack[n].drop_distance(other))
            .min()
        {
            stack[n].z.start -= drop;
            stack[n].z.end -= drop;
        } else if stack[n].z.start > 1 {
            let drop = stack[n].z.start - 1;
            stack[n].z.start -= drop;
            stack[n].z.end -= drop;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    id: usize,
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}

impl Brick {
    fn parse(line: &str, id: usize) -> Self {
        let (lhs, rhs) = line.split_once('~').unwrap();
        let lhs = lhs.split(',').map(|s| s.parse::<usize>().unwrap());
        let rhs = rhs.split(',').map(|s| s.parse::<usize>().unwrap());
        let mut ranges = lhs.zip(rhs).map(|(a, b)| a.min(b)..a.max(b) + 1);
        Self {
            id,
            x: ranges.next().unwrap(),
            y: ranges.next().unwrap(),
            z: ranges.next().unwrap(),
        }
    }

    /// Returns the number of bricks that would fall if this brick was removed.
    fn chain_reaction_count(&self, graph: &[(usize, Vec<usize>)]) -> usize {
        graph
            .iter()
            .filter(|(_, supporters)| !supporters.is_empty())
            .fold(
                HashSet::from([self.id]),
                |mut removed, (brick, supporters)| {
                    if supporters
                        .iter()
                        .all(|supporter| removed.contains(supporter))
                    {
                        removed.insert(*brick);
                    }
                    removed
                },
            )
            .len()
            - 1
    }

    /// Returns ids of all bricks that are supporting this brick.
    fn get_supporters(&self, stack: &[Brick]) -> Vec<usize> {
        stack
            .iter()
            .filter(|other| other.is_supporting(self))
            .map(|other| other.id)
            .collect()
    }

    /// Returns `true` if this brick is supporting the other brick.
    fn is_supporting(&self, other: &Self) -> bool {
        self.z.end == other.z.start
            && self.x.start < other.x.end
            && self.x.end > other.x.start
            && self.y.start < other.y.end
            && self.y.end > other.y.start
    }

    /// Returns the distance this brick could drop before it would collide with the other brick.
    /// Returns `None` if this brick is not above the other brick.
    fn drop_distance(&self, other: &Self) -> Option<usize> {
        if self.z.start >= other.z.end
            && self.x.start < other.x.end
            && self.x.end > other.x.start
            && self.y.start < other.y.end
            && self.y.end > other.y.start
        {
            Some(self.z.start - other.z.end)
        } else {
            None
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
        assert_eq!(result, 7);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 39933);
    }
}
