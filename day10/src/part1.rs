use std::iter::from_fn;

pub fn solve(input: &str) -> usize {
    let map = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut locations = find_start_directions(&map);
    from_fn(move || {
        locations = (
            find_next_location(locations.0, &map),
            find_next_location(locations.1, &map),
        );
        Some(locations)
    })
    .enumerate()
    .find(|(_, (l1, l2))| l1.row == l2.row && l1.column == l2.column)
    .unwrap()
    .0 + 2
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

fn find_start_directions(map: &[&[u8]]) -> (Location, Location) {
    let (row, column) = find_start(map);
    let north = match map[row.saturating_sub(1)][column] {
        b'|' | b'7' | b'F' => Some(Location {
            row: row - 1,
            column,
            from: Direction::South,
        }),
        _ => None,
    };
    let south = match map[(row + 1).clamp(0, map.len())][column] {
        b'|' | b'L' | b'J' => Some(Location {
            row: row + 1,
            column,
            from: Direction::North,
        }),
        _ => None,
    };
    let east = match map[row][(column + 1).clamp(0, map[row].len())] {
        b'-' | b'J' | b'7' => Some(Location {
            row,
            column: column + 1,
            from: Direction::West,
        }),
        _ => None,
    };
    let west = match map[row][column.saturating_sub(1)] {
        b'-' | b'L' | b'F' => Some(Location {
            row,
            column: column - 1,
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
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 8);
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
