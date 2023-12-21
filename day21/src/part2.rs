use arrayvec::ArrayVec;
use grid::Grid;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    reachable_after(input, 26501365)
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

fn is_valid_pos(pos: (isize, isize), grid: &Grid<u8>) -> bool {
    let row = pos.0.rem_euclid(grid.rows() as isize) as usize;
    let col = pos.1.rem_euclid(grid.cols() as isize) as usize;
    grid.get(row, col) != Some(&b'#')
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_after_6_steps() {
        let result = reachable_after(EXAMPLE, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn example_after_10_steps() {
        let result = reachable_after(EXAMPLE, 10);
        assert_eq!(result, 50);
    }

    #[test]
    fn example_after_50_steps() {
        let result = reachable_after(EXAMPLE, 50);
        assert_eq!(result, 1594);
    }

    #[test]
    fn example_after_100_steps() {
        let result = reachable_after(EXAMPLE, 100);
        assert_eq!(result, 6536);
    }

    #[test]
    fn example_after_500_steps() {
        let result = reachable_after(EXAMPLE, 500);
        assert_eq!(result, 167004);
    }

    #[test]
    fn example_after_1000_steps() {
        let result = reachable_after(EXAMPLE, 1000);
        assert_eq!(result, 668697);
    }

    #[test]
    #[ignore = "not done yet"]
    fn example_after_5000_steps() {
        let result = reachable_after(EXAMPLE, 5000);
        assert_eq!(result, 16733044);
    }

    #[test]
    #[ignore = "not done yet"]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 42);
    }
}
