use grid::Grid;

pub fn solve(input: &str) -> usize {
    get_north_load_after_cycles(input, 1000000000)
}

fn get_north_load_after_cycles(input: &str, cycles: usize) -> usize {
    let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let width = map[0].len();
    let mut grid = Grid::from_vec(map.into_iter().flatten().copied().collect(), width);

    let mut history = vec![];
    for n in 0..cycles {
        roll_cycle(&mut grid);

        if let Some(prev) = history.iter().position(|other| *other == grid) {
            let pos = prev + ((cycles - prev - 1) % (n - prev));
            return north_load(&history[pos]);
        }

        history.push(grid.clone());
    }

    north_load(&grid)
}

fn roll_cycle(grid: &mut Grid<u8>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn roll_north(grid: &mut Grid<u8>) {
    for col in 0..grid.cols() {
        let mut insert_row = 0;
        for row in 0..grid.rows() {
            match grid[(row, col)] {
                b'O' => {
                    if insert_row != row {
                        grid[(insert_row, col)] = b'O';
                        grid[(row, col)] = b'.';
                    }
                    insert_row += 1;
                }
                b'#' => {
                    insert_row = row + 1;
                }
                _ => (),
            }
        }
    }
}

fn roll_west(grid: &mut Grid<u8>) {
    for row in 0..grid.rows() {
        let mut insert_col = 0;
        for col in 0..grid.cols() {
            match grid[(row, col)] {
                b'O' => {
                    if insert_col != col {
                        grid[(row, insert_col)] = b'O';
                        grid[(row, col)] = b'.';
                    }
                    insert_col += 1;
                }
                b'#' => {
                    insert_col = col + 1;
                }
                _ => (),
            }
        }
    }
}

fn roll_south(grid: &mut Grid<u8>) {
    for col in 0..grid.cols() {
        let mut insert_row = grid.rows() - 1;
        for row in (0..grid.rows()).rev() {
            match grid[(row, col)] {
                b'O' => {
                    if insert_row != row {
                        grid[(insert_row, col)] = b'O';
                        grid[(row, col)] = b'.';
                    }
                    insert_row = insert_row.saturating_sub(1);
                }
                b'#' => {
                    insert_row = row.saturating_sub(1);
                }
                _ => (),
            }
        }
    }
}

fn roll_east(grid: &mut Grid<u8>) {
    for row in 0..grid.rows() {
        let mut insert_col = grid.cols() - 1;
        for col in (0..grid.cols()).rev() {
            match grid[(row, col)] {
                b'O' => {
                    if insert_col != col {
                        grid[(row, insert_col)] = b'O';
                        grid[(row, col)] = b'.';
                    }
                    insert_col = insert_col.saturating_sub(1);
                }
                b'#' => {
                    insert_col = col.saturating_sub(1);
                }
                _ => (),
            }
        }
    }
}

fn north_load(grid: &Grid<u8>) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(row, data)| data.filter(|c| **c == b'O').count() * (grid.rows() - row))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 64);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
