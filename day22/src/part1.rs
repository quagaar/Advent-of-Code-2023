use std::ops::Range;

pub fn solve(input: &str) -> usize {
    let mut stack = input.lines().map(Brick::parse).collect::<Vec<_>>();

    settle_stack(&mut stack);

    stack
        .iter()
        .filter(|brick| brick.is_safe_to_disintegrate(&stack))
        .count()
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

struct Brick {
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}

impl Brick {
    fn parse(line: &str) -> Self {
        let (lhs, rhs) = line.split_once('~').unwrap();
        let lhs = lhs.split(',').map(|s| s.parse::<usize>().unwrap());
        let rhs = rhs.split(',').map(|s| s.parse::<usize>().unwrap());
        let mut ranges = lhs.zip(rhs).map(|(a, b)| a.min(b)..a.max(b) + 1);
        Self {
            x: ranges.next().unwrap(),
            y: ranges.next().unwrap(),
            z: ranges.next().unwrap(),
        }
    }

    /// Returns `true` if disintegrating this brick would not cause the stack to collapse.
    fn is_safe_to_disintegrate(&self, stack: &[Brick]) -> bool {
        stack
            .iter()
            .filter(|other| self.is_supporting(other))
            .all(|other| other.count_supporters(stack) > 1)
    }

    /// Returns the number of bricks that are supporting this brick.
    fn count_supporters(&self, stack: &[Brick]) -> usize {
        stack
            .iter()
            .filter(|other| other.is_supporting(self))
            .count()
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
        assert_eq!(result, 5);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 386);
    }
}
