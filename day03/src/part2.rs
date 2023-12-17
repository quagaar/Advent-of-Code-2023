use itertools::{chain, Itertools};
use std::{iter::from_fn, ops::Range};

pub fn solve_part2(input: &str) -> usize {
    chain!([""], input.lines(), [""])
        .tuple_windows()
        .fold(0, |acc, (prev, cur, next)| {
            acc + find_gear_positions(cur)
                .filter_map(gear_ratio_calculator(prev, cur, next))
                .sum::<usize>()
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

const MAX_DIGITS: usize = 3; // Maximum number of digits in a number

fn gear_ratio_calculator<'a>(
    prev: &'a str,
    cur: &'a str,
    next: &'a str,
) -> impl Fn(usize) -> Option<usize> + 'a {
    |gear_pos| {
        let start = gear_pos.saturating_sub(MAX_DIGITS);
        let end = (gear_pos + MAX_DIGITS + 1).min(cur.len());
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

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 467835);
    }

    #[test]
    fn result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 79844424);
    }
}
