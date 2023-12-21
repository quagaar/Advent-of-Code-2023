use arrayvec::ArrayVec;
use grid::Grid;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    reachable_after(input, 64)
}

fn reachable_after(input: &str, steps: usize) -> usize {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let cols = lines[0].len();
    let grid = Grid::from_vec(lines.into_iter().flatten().copied().collect(), cols);
    let start = grid.indexed_iter().find(|(_, &c)| c == b'S').unwrap().0;
    let start = (start.0 as isize, start.1 as isize);

    (0..steps)
        .fold(HashSet::from([start]), |acc, _| {
            acc.into_iter()
                .flat_map(|pos| reachable_neighbors(pos, &grid))
                .collect()
        })
        .len()
}

fn reachable_neighbors(pos: (isize, isize), grid: &Grid<u8>) -> ArrayVec<(isize, isize), 4> {
    [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
    .into_iter()
    .filter(|&pos| is_valid_pos(pos, grid))
    .collect()
}

fn is_valid_pos((row, col): (isize, isize), grid: &Grid<u8>) -> bool {
    matches!(grid.get(row, col), Some(&b'.') | Some(b'S'))
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = reachable_after(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 3733);
    }
}
