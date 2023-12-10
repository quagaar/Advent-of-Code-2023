use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let map = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut pipe = HashSet::new();

    let start = find_start(&map);
    pipe.insert(start);

    let (mut l1, mut l2) = find_start_directions(&map, start);
    let start_pipe = match (l1.from, l2.from) {
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => b'|',
        (Direction::East, Direction::West) | (Direction::West, Direction::East) => b'-',
        (Direction::South, Direction::West) | (Direction::West, Direction::South) => b'L',
        (Direction::North, Direction::West) | (Direction::West, Direction::North) => b'F',
        (Direction::South, Direction::East) | (Direction::East, Direction::South) => b'J',
        (Direction::North, Direction::East) | (Direction::East, Direction::North) => b'7',
        _ => unreachable!(),
    };
    while l1.row != l2.row || l1.column != l2.column {
        pipe.insert((l1.row, l1.column));
        pipe.insert((l2.row, l2.column));
        l1 = find_next_location(l1, &map);
        l2 = find_next_location(l2, &map);
    }
    pipe.insert((l1.row, l1.column));

    map.into_iter()
        .enumerate()
        .map(|(row, map_line)| {
            map_line
                .iter()
                .enumerate()
                .fold(
                    (0, false, b'.'),
                    |(count, inside, pipe_state), (column, cell)| {
                        if pipe.contains(&(row, column)) {
                            let real_cell = if *cell == b'S' { start_pipe } else { *cell };
                            match real_cell {
                                b'|' => (count, !inside, b'.'),
                                b'-' => (count, inside, pipe_state),
                                b'L' => (count, !inside, b'L'),
                                b'F' => (count, !inside, b'F'),
                                b'J' => {
                                    if pipe_state == b'F' {
                                        (count, inside, b'.')
                                    } else {
                                        (count, !inside, b'.')
                                    }
                                }
                                b'7' => {
                                    if pipe_state == b'L' {
                                        (count, inside, b'.')
                                    } else {
                                        (count, !inside, b'.')
                                    }
                                }
                                _ => unreachable!(),
                            }
                        } else if inside {
                            (count + 1, inside, b'.')
                        } else {
                            (count, inside, b'.')
                        }
                    },
                )
                .0
        })
        .sum()
}

fn find_start(map: &[&[u8]]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(row, map_line)| {
            map_line
                .iter()
                .position(|cell| *cell == b'S')
                .map(|column| (row, column))
        })
        .expect("Start not found in map")
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Location {
    row: usize,
    column: usize,
    from: Direction,
}

fn find_start_directions(
    map: &[&[u8]],
    (start_row, start_column): (usize, usize),
) -> (Location, Location) {
    let north = match map[start_row.saturating_sub(1)][start_column] {
        b'|' | b'7' | b'F' => Some(Location {
            row: start_row - 1,
            column: start_column,
            from: Direction::South,
        }),
        _ => None,
    };
    let south = match map[(start_row + 1).clamp(0, map.len())][start_column] {
        b'|' | b'L' | b'J' => Some(Location {
            row: start_row + 1,
            column: start_column,
            from: Direction::North,
        }),
        _ => None,
    };
    let east = match map[start_row][(start_column + 1).clamp(0, map[start_row].len())] {
        b'-' | b'J' | b'7' => Some(Location {
            row: start_row,
            column: start_column + 1,
            from: Direction::West,
        }),
        _ => None,
    };
    let west = match map[start_row][start_column.saturating_sub(1)] {
        b'-' | b'L' | b'F' => Some(Location {
            row: start_row,
            column: start_column - 1,
            from: Direction::East,
        }),
        _ => None,
    };
    let mut results = [north, south, east, west].into_iter().flatten();
    (
        results.next().expect("No pipes from start!"),
        results.next().expect("Only one pipe from start!"),
    )
}

fn find_next_location(location: Location, map: &[&[u8]]) -> Location {
    let pipe = map[location.row][location.column];
    match (location.from, pipe) {
        (Direction::North, b'|') => Location {
            row: location.row + 1,
            column: location.column,
            from: Direction::North,
        },
        (Direction::North, b'L') => Location {
            row: location.row,
            column: location.column + 1,
            from: Direction::West,
        },
        (Direction::North, b'J') => Location {
            row: location.row,
            column: location.column - 1,
            from: Direction::East,
        },

        (Direction::South, b'|') => Location {
            row: location.row - 1,
            column: location.column,
            from: Direction::South,
        },
        (Direction::South, b'7') => Location {
            row: location.row,
            column: location.column - 1,
            from: Direction::East,
        },
        (Direction::South, b'F') => Location {
            row: location.row,
            column: location.column + 1,
            from: Direction::West,
        },

        (Direction::East, b'-') => Location {
            row: location.row,
            column: location.column - 1,
            from: Direction::East,
        },
        (Direction::East, b'L') => Location {
            row: location.row - 1,
            column: location.column,
            from: Direction::South,
        },
        (Direction::East, b'F') => Location {
            row: location.row + 1,
            column: location.column,
            from: Direction::North,
        },

        (Direction::West, b'-') => Location {
            row: location.row,
            column: location.column + 1,
            from: Direction::West,
        },
        (Direction::West, b'7') => Location {
            row: location.row + 1,
            column: location.column,
            from: Direction::North,
        },
        (Direction::West, b'J') => Location {
            row: location.row - 1,
            column: location.column,
            from: Direction::South,
        },

        _ => panic!("Impossible location: {:?}", location),
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE3: &str = include_str!("../example3.txt");
    const EXAMPLE4: &str = include_str!("../example4.txt");
    const EXAMPLE5: &str = include_str!("../example5.txt");

    #[test]
    fn example3() {
        let result = solve(EXAMPLE3);
        assert_eq!(result, 4);
    }

    #[test]
    fn example4() {
        let result = solve(EXAMPLE4);
        assert_eq!(result, 8);
    }

    #[test]
    fn example5() {
        let result = solve(EXAMPLE5);
        assert_eq!(result, 10);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 381);
    }
}
