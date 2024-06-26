use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let directions = input.lines().next().unwrap();
    let map = input
        .lines()
        .skip(2)
        .filter_map(read_map_line)
        .collect::<HashMap<_, _>>();
    let mut location = "AAA";
    for (n, direction) in directions.chars().cycle().enumerate() {
        let next = map.get(location).expect("Unknown location");
        match direction {
            'L' => location = next.0,
            'R' => location = next.1,
            other => panic!("Invalid direction: {}", other),
        }
        if location == "ZZZ" {
            return n + 1;
        }
    }
    unreachable!()
}

fn read_map_line(line: &str) -> Option<(&str, (&str, &str))> {
    let (loc, next) = line.split_once(" = ")?;
    let (left, right) = next.trim_matches(['(', ')'].as_ref()).split_once(", ")?;
    Some((loc, (left, right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example1() {
        let result = solve(EXAMPLE1);
        assert_eq!(result, 2);
    }

    #[test]
    fn example2() {
        let result = solve(EXAMPLE2);
        assert_eq!(result, 6);
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
