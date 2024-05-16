use itertools::{chain, Itertools};
use std::{iter::from_fn, ops::Range};

pub fn solve(input: &str) -> usize {
    chain!([""], input.lines(), [""])
        .tuple_windows()
        .fold(0, |acc, (prev, cur, next)| {
            acc + find_number_ranges(cur)
                .filter(part1_validator(prev, cur, next))
                .filter_map(|r| cur[r].parse::<usize>().ok())
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

fn part1_validator<'a>(
    prev: &'a str,
    cur: &'a str,
    next: &'a str,
) -> impl Fn(&Range<usize>) -> bool + 'a {
    |r| {
        let start = r.start.saturating_sub(1);
        let end = (r.end + 1).min(cur.len());

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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 4361);
    }

    #[cfg(input_txt)]
    #[cfg(part1_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part1.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
