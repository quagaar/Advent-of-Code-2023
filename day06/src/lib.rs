pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = get_data_line(lines.next().unwrap(), "Time");
    let distances = get_data_line(lines.next().unwrap(), "Distance");
    times
        .into_iter()
        .zip(distances)
        .map(count_win_scenarios)
        .product()
}

pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = get_value_line(lines.next().unwrap(), "Time");
    let distance = get_value_line(lines.next().unwrap(), "Distance");
    count_win_scenarios((time, distance))
}

fn get_data_line<'a>(line: &'a str, expected: &str) -> impl Iterator<Item = usize> + 'a {
    let (label, data) = line.split_once(':').unwrap();
    debug_assert!(label == expected);
    data.split_ascii_whitespace().filter_map(|s| s.parse().ok())
}

fn get_value_line(line: &str, expected: &str) -> usize {
    let (label, data) = line.split_once(':').unwrap();
    debug_assert!(label == expected);
    String::from_iter(data.split_ascii_whitespace().flat_map(|s| s.chars()))
        .parse()
        .unwrap()
}

fn count_win_scenarios((race_length, record): (usize, usize)) -> usize {
    let mut low = 0;
    let mut high = race_length / 2;

    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if is_winner(race_length, mid, record) {
            high = mid;
        } else {
            low = mid;
        }
    }

    race_length + 1 - high * 2
}

fn is_winner(race_length: usize, hold_time: usize, record: usize) -> bool {
    (race_length - hold_time) * hold_time > record
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
