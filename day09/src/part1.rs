use rayon::prelude::*;

pub fn solve(input: &str) -> i32 {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> i32 {
    find_next_value(
        line.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect(),
    )
}

fn find_next_value(numbers: Vec<i32>) -> i32 {
    let last = numbers.last().copied().unwrap();
    let diffs = numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    if diffs.iter().all(|x| *x == 0) {
        last
    } else {
        last + find_next_value(diffs)
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
        assert_eq!(result, 114);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 1938800261);
    }
}
