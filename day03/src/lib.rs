use itertools::{chain, Itertools};
use std::{iter::from_fn, ops::Range};

pub fn solve_part1(input: &str) -> usize {
    chain!([""], input.lines(), [""])
        .tuple_windows()
        .fold(0, |mut acc, (prev, cur, next)| {
            acc += find_number_ranges(cur)
                .filter(part1_validator(prev, cur, next))
                .filter_map(|r| cur[r].parse::<usize>().ok())
                .sum::<usize>();
            acc
        })
}

pub fn solve_part2(input: &str) -> usize {
    chain!([""], input.lines(), [""])
        .tuple_windows()
        .fold(0, |mut acc, (prev, cur, next)| {
            acc += find_gear_positions(cur)
                .filter_map(gear_ratio_calculator(prev, cur, next))
                .sum::<usize>();
            acc
        })
}

fn find_number_ranges(line: &str) -> impl Iterator<Item = Range<usize>> + '_ {
    let mut pos = 0;
    from_fn(move || {
        if let Some(offset) = line[pos..].find(|c: char| c.is_ascii_digit()) {
            let start = pos + offset;
            if let Some(len) = line[start..].find(|c: char| !c.is_ascii_digit()) {
                pos = start + len;
            } else {
                pos = line.len();
            }
            Some(start..pos)
        } else {
            None
        }
    })
}

fn find_gear_positions(line: &str) -> impl Iterator<Item = usize> + '_ {
    let mut pos = 0;
    from_fn(move || {
        if let Some(offset) = line[pos..].find(|c| c == '*') {
            let result = pos + offset;
            pos = result + 1;
            Some(result)
        } else {
            None
        }
    })
}

fn find_adjacent_numbers(line: &str, gear_pos: usize) -> impl Iterator<Item = usize> + '_ {
    find_number_ranges(line)
        .filter(move |r| r.start <= gear_pos + 1 && r.end >= gear_pos)
        .filter_map(|r| line[r].parse::<usize>().ok())
}

fn part1_validator<'a>(
    prev: &'a str,
    cur: &'a str,
    next: &'a str,
) -> impl Fn(&Range<usize>) -> bool + 'a {
    |r| {
        let start = r.start.saturating_sub(1);
        let end = (r.end + 1).clamp(0, cur.len());

        has_symbol(prev.get(start..end))
            || has_symbol(next.get(start..end))
            || has_symbol(cur.get(start..end))
    }
}

fn has_symbol(s: Option<&str>) -> bool {
    if let Some(s) = s {
        s.chars().any(is_symbol)
    } else {
        false
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

const MAX_DIGITS: usize = 3; // Maximum number of digits in a number

fn gear_ratio_calculator<'a>(
    prev: &'a str,
    cur: &'a str,
    next: &'a str,
) -> impl Fn(usize) -> Option<usize> + 'a {
    |gear_pos| {
        let start = gear_pos.saturating_sub(MAX_DIGITS);
        let end = (gear_pos + MAX_DIGITS + 1).clamp(0, cur.len());
        let (count, ratio) = chain!(
            find_adjacent_numbers(&prev[start..end], gear_pos - start),
            find_adjacent_numbers(&cur[start..end], gear_pos - start),
            find_adjacent_numbers(&next[start..end], gear_pos - start)
        )
        .fold((0, 1), |(count, ratio), number| (count + 1, ratio * number));
        if count == 2 {
            Some(ratio)
        } else {
            None
        }
    }
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 535235);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 467835);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 79844424);
    }
}
