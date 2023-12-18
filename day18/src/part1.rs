use itertools::Itertools;

pub fn solve(input: &str) -> u32 {
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
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trench {
    end: Position,
    length: u32,
}

impl Trench {
    fn create(position: &mut Position, line: &str) -> Option<Self> {
        let direction = line.chars().next().unwrap();
        let distance: u32 = line[2..].split_once(' ').unwrap().0.parse().unwrap();
        match direction {
            'U' => {
                position.y += distance as i32;
            }
            'D' => {
                position.y -= distance as i32;
            }
            'L' => {
                position.x -= distance as i32;
            }
            'R' => {
                position.x += distance as i32;
            }
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
        assert_eq!(result, 62);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 34329);
    }
}
