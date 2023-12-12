use rayon::prelude::*;
use std::{collections::HashMap, iter::from_fn};

pub fn solve(input: &str) -> usize {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (pattern, counts) = line.split_once(' ').unwrap();

    let pattern = format!("{0}?{0}?{0}?{0}?{0}", pattern);
    let counts = format!("{0},{0},{0},{0},{0}", counts);

    counts
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .fold(initial_states(pattern.as_str()), |acc, count| {
            let mut memo = HashMap::new();

            acc.filter(move |(s, _)| has_count_hashes(s, count))
                .map(move |(s, n)| (&s[count..], n))
                .for_each(|(s, n)| {
                    for after in after_hashes_states(s) {
                        *memo.entry(after).or_insert(0) += n;
                    }
                });

            Box::new(memo.into_iter())
        })
        .filter(|(s, _)| s.is_empty())
        .map(|(_, n)| n)
        .sum()
}

fn initial_states<'a>(pattern: &'a str) -> Box<dyn Iterator<Item = (&'a str, usize)> + 'a> {
    Box::new(
        (0..pattern.len())
            .take_while(|n| *n == 0 || pattern.chars().nth(n - 1) != Some('#'))
            .map(|n| (&pattern[n..], 1)),
    )
}

fn has_count_hashes(pattern: &str, count: usize) -> bool {
    (0..count).all(|n| matches!(pattern.chars().nth(n), Some('#') | Some('?')))
}

fn after_hashes_states(pattern: &str) -> impl Iterator<Item = &str> {
    let mut n = 0;
    from_fn(move || {
        if pattern.is_empty() {
            if n == 0 {
                n += 1;
                Some(pattern)
            } else {
                None
            }
        } else {
            match pattern.chars().nth(n) {
                Some('?') | Some('.') => {
                    n += 1;
                    Some(&pattern[n..])
                }
                _ => None,
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 525152);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 1909291258644);
    }
}
