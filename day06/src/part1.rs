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

fn get_data_line<'a>(line: &'a str, expected: &str) -> impl Iterator<Item = usize> + 'a {
    let (label, data) = line.split_once(':').unwrap();
    debug_assert!(label == expected);
    data.split_ascii_whitespace().filter_map(|s| s.parse().ok())
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 4403592);
    }
}
