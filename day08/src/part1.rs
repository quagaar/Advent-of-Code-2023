use std::collections::HashMap;

pub fn solve_part1(input: &str) -> usize {
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
    use super::super::*;
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_example2() {
        let result = solve_part1(EXAMPLE2);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 21797);
    }
}
