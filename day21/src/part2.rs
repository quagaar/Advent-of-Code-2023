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

    let mut positions = HashSet::from([start]);

    let repeats = steps / grid.rows();

    if repeats < 1 {
        for _ in 0..steps {
            positions = next_steps(positions, &grid);
        }
        return positions.len();
    }

    let init = steps % grid.rows();

    for _ in 0..init {
        positions = next_steps(positions, &grid);
    }

    let mut samples = vec![positions.len()];

    for _ in 0..grid.rows() {
        positions = next_steps(positions, &grid);
    }

    samples.push(positions.len());

    for n in 1..repeats {
        for _ in 0..grid.rows() {
            positions = next_steps(positions, &grid);
        }
        samples.push(positions.len());

        if let Some(result) = try_find_result(&samples, repeats - n) {
            return result;
        }
    }

    positions.len()
}

fn next_steps(current: HashSet<(isize, isize)>, grid: &Grid<u8>) -> HashSet<(isize, isize)> {
    current
        .into_iter()
        .flat_map(|pos| reachable_neighbors(pos, grid))
        .collect()
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

fn try_find_result(samples: &[usize], remaining_repeats: usize) -> Option<usize> {
    let samples = samples.iter().map(|&n| n as isize).collect::<Vec<_>>();
    if let Some(diffs) = try_find_next(&samples) {
        (1..remaining_repeats)
            .fold(diffs, |diffs, _| {
                diffs
                    .iter()
                    .scan(0, |state, &n| {
                        *state += n;
                        Some(*state)
                    })
                    .collect()
            })
            .last()
            .map(|n| *n as usize)
    } else {
        None
    }
}

fn try_find_next(sequence: &[isize]) -> Option<Vec<isize>> {
    if sequence.len() < 2 {
        None
    } else if sequence.windows(2).all(|w| w[0] == w[1]) {
        Some(vec![sequence[0]])
    } else {
        let diffs = sequence.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        try_find_next(&diffs).map(|mut diffs| {
            diffs.push(*sequence.last().unwrap());
            diffs
        })
    }
}

#[cfg(test)]
mod tests {
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
    #[ignore = "Takes too long to run"]
    fn example_after_1000_steps() {
        let result = reachable_after(EXAMPLE, 1000);
        assert_eq!(result, 668697);
    }

    #[test]
    #[ignore = "Takes too long to run"]
    fn example_after_5000_steps() {
        let result = reachable_after(EXAMPLE, 5000);
        assert_eq!(result, 16733044);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    #[ignore = "Takes too long to run"]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
