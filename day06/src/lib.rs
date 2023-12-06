pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = get_data_line(lines.next().unwrap(), "Time");
    let distances = get_data_line(lines.next().unwrap(), "Distance");
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| (0..time).filter(|x| (time - x) * x > distance).count())
        .product()
}

pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = get_value_line(lines.next().unwrap(), "Time");
    let distance = get_value_line(lines.next().unwrap(), "Distance");
    (0..time).filter(|x| (time - x) * x > distance).count()
}

fn get_data_line(line: &str, expected: &str) -> Vec<usize> {
    let (label, data) = line.split_once(':').unwrap();
    debug_assert!(label == expected);
    data.split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_value_line(line: &str, expected: &str) -> usize {
    let (label, data) = line.split_once(':').unwrap();
    debug_assert!(label == expected);
    String::from_iter(data.split_ascii_whitespace().flat_map(|s| s.chars()))
        .parse()
        .unwrap()
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 4403592);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 38017587);
    }
}
