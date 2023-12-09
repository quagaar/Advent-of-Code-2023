use rayon::prelude::*;

pub fn solve_part2(input: &str) -> i32 {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> i32 {
    find_previous_value(
        line.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect(),
    )
}

fn find_previous_value(numbers: Vec<i32>) -> i32 {
    let first = numbers.first().copied().unwrap();
    let diffs = numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    if diffs.iter().all(|x| *x == 0) {
        first
    } else {
        first - find_previous_value(diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 1112);
    }
}
