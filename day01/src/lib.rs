#[macro_use]
extern crate lazy_static;

use rayon::prelude::*;
use regex::Regex;

pub fn solve_part1(input: &str) -> u32 {
    input
        .par_lines()
        .filter_map(|line| {
            if let (Some(first), Some(last)) = (
                line.chars().find_map(|c| c.to_digit(10)),
                line.chars().rev().find_map(|c| c.to_digit(10)),
            ) {
                Some(10 * first + last)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .par_lines()
        .filter_map(|line| {
            if let (Some(first), Some(last)) = (first_digit(line), last_digit(line)) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .sum()
}

lazy_static! {
    static ref FIRST_REGEX: Regex = Regex::new(
        r"(one|1)|(two|2)|(three|3)|(four|4)|(five|5)|(six|6)|(seven|7)|(eight|8)|(nine|9)"
    )
    .unwrap();
    static ref LAST_REGEX: Regex = Regex::new(
        r"^.*(?:(one|1)|(two|2)|(three|3)|(four|4)|(five|5)|(six|6)|(seven|7)|(eight|8)|(nine|9))"
    )
    .unwrap();
}

fn first_digit(line: &str) -> Option<u32> {
    FIRST_REGEX
        .captures(line)?
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, x)| x.is_some())
        .map(|(digit, _)| digit as u32)
        .next()
}

fn last_digit(line: &str) -> Option<u32> {
    LAST_REGEX
        .captures(line)?
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, x)| x.is_some())
        .map(|(digit, _)| digit as u32)
        .next()
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const EXAMPLE2: &str = include_str!("../example2.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 142);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 55108);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE2);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 56324);
    }
}
