use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn solve(input: &str) -> usize {
    get_north_load_after_cycles(input, 1000000000)
}

fn get_north_load_after_cycles(input: &str, cycles: usize) -> usize {
    let mut map: Vec<Vec<u8>> = input.lines().map(|s| s.as_bytes().to_vec()).collect();

    let mut history = vec![];
    for n in 0..cycles {
        roll_cycle(&mut map);

        let mut hasher = DefaultHasher::new();
        map.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some((prev, _)) = history
            .iter()
            .enumerate()
            .find(|(_, (other, _))| *other == hash)
        {
            let pos = prev + ((cycles - prev) % (n - prev)) - 1;
            return history[pos].1;
        }

        history.push((hash, north_load(&map)));
    }

    history.last().unwrap().1
}

fn roll_cycle(map: &mut Vec<Vec<u8>>) {
    roll_north(map);
    roll_west(map);
    roll_south(map);
    roll_east(map);
}

fn roll_north(map: &mut Vec<Vec<u8>>) {
    for col in 0..map[0].len() {
        let mut insert_row = 0;
        for row in 0..map.len() {
            match map[row][col] {
                b'O' => {
                    if insert_row != row {
                        map[insert_row][col] = b'O';
                        map[row][col] = b'.';
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

fn roll_west(map: &mut [Vec<u8>]) {
    for row in map.iter_mut() {
        let mut insert_col = 0;
        for col in 0..row.len() {
            match row[col] {
                b'O' => {
                    if insert_col != col {
                        row[insert_col] = b'O';
                        row[col] = b'.';
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

fn roll_south(map: &mut Vec<Vec<u8>>) {
    for col in 0..map[0].len() {
        let mut insert_row = map.len() - 1;
        for row in (0..map.len()).rev() {
            match map[row][col] {
                b'O' => {
                    if insert_row != row {
                        map[insert_row][col] = b'O';
                        map[row][col] = b'.';
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

fn roll_east(map: &mut [Vec<u8>]) {
    for row in map.iter_mut() {
        let mut insert_col = row.len() - 1;
        for col in (0..row.len()).rev() {
            match row[col] {
                b'O' => {
                    if insert_col != col {
                        row[insert_col] = b'O';
                        row[col] = b'.';
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

fn north_load(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(row, data)| data.iter().filter(|c| **c == b'O').count() * (map.len() - row))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 64);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 87700);
    }
}
