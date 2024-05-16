pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let time = get_value_line(lines.next().unwrap(), "Time");
    let distance = get_value_line(lines.next().unwrap(), "Distance");
    count_win_scenarios((time, distance))
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 71503);
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
