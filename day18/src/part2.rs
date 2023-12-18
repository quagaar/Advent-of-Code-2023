use itertools::Itertools;

pub fn solve(input: &str) -> u64 {
    let trenches = input
        .lines()
        .scan(Position { x: 0, y: 0 }, Trench::create)
        .collect::<Vec<_>>();

    // Use the shoelace algorithm to calculate the area of a polygon.
    // The polygon lines go down the center of the trenches, so we have to
    // add the half of area of the trenches plus one to correct for the
    // width of the trenches.

    let (perimeter, sum1, sum2) = trenches.iter().circular_tuple_windows().fold(
        (0, 0, 0),
        |(perimeter, sum1, sum2), (current, next)| {
            let perimeter = perimeter + current.length;
            let sum1 = sum1 + current.end.x * next.end.y;
            let sum2 = sum2 + current.end.y * next.end.x;
            (perimeter, sum1, sum2)
        },
    );

    1 + (sum1.abs_diff(sum2) + perimeter) / 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trench {
    end: Position,
    length: u64,
}

impl Trench {
    fn create(position: &mut Position, line: &str) -> Option<Self> {
        let (_, hex) = line.split_once(" (#").unwrap();
        let distance = u64::from_str_radix(&hex[0..5], 16).unwrap();
        let direction = hex.chars().nth(5).unwrap();
        match direction {
            '3' => position.y += distance as i64,
            '1' => position.y -= distance as i64,
            '2' => position.x -= distance as i64,
            '0' => position.x += distance as i64,
            _ => panic!("Unknown direction: {}", direction),
        }
        Some(Self {
            end: *position,
            length: distance,
        })
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
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 42617947302920);
    }
}
