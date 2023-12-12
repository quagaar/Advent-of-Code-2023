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
            acc.into_iter()
                .filter(|(n, _)| has_count_hashes(&pattern[*n..], count))
                .map(|(n, m)| (n + count, m))
                .flat_map(|(n, m)| {
                    after_hashes_positions(&pattern[n..]).map(move |pos| (n + pos, m))
                })
                .fold(HashMap::new(), |mut acc, (n, m)| {
                    acc.entry(n).and_modify(|x| *x += m).or_insert(m);
                    acc
                })
        })
        .into_iter()
        .filter(|(n, _)| *n == pattern.len())
        .map(|(_, n)| n)
        .sum()
}

fn initial_states(pattern: &str) -> HashMap<usize, usize> {
    (0..pattern.len())
        .take_while(|n| *n == 0 || pattern.chars().nth(n - 1) != Some('#'))
        .map(|n| (n, 1))
        .collect()
}

fn has_count_hashes(pattern: &str, count: usize) -> bool {
    (0..count).all(|n| matches!(pattern.chars().nth(n), Some('#') | Some('?')))
        && pattern.chars().nth(count) != Some('#')
}

fn after_hashes_positions(pattern: &str) -> impl Iterator<Item = usize> + '_ {
    let mut pos = Some(0);
    from_fn(move || {
        if let Some(mut n) = pos {
            if pattern.is_empty() {
                pos = None;
                Some(0)
            } else {
                loop {
                    n += 1;
                    match pattern.chars().nth(n) {
                        Some('.') => continue,
                        Some('?') => {
                            pos = Some(n);
                            break Some(n);
                        }
                        _ => {
                            pos = None;
                            break Some(n);
                        }
                    }
                }
            }
        } else {
            None
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
