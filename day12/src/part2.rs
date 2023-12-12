use rayon::prelude::*;
use std::{collections::HashMap, iter::from_fn};

pub fn solve(input: &str) -> usize {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (pattern, group_sizes) = line.split_once(' ').unwrap();

    let pattern = format!("{0}?{0}?{0}?{0}?{0}", pattern);
    let group_sizes = format!("{0},{0},{0},{0},{0}", group_sizes);

    group_sizes
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .fold(initial_states(pattern.as_str()), |acc, group_size| {
            acc.into_iter()
                .filter(|(start_pos, _)| is_valid_group(&pattern[*start_pos..], group_size))
                .map(|(start_pos, count)| (start_pos + group_size, count))
                .flat_map(|(pos, count)| {
                    after_hashes_positions(&pattern[pos..]).map(move |offset| (pos + offset, count))
                })
                .fold(HashMap::new(), |mut acc, (pos, count)| {
                    acc.entry(pos)
                        .and_modify(|total| *total += count)
                        .or_insert(count);
                    acc
                })
        })
        .into_iter()
        .filter(|(pos, _)| *pos == pattern.len())
        .map(|(_, count)| count)
        .sum()
}

fn initial_states(pattern: &str) -> HashMap<usize, usize> {
    (0..pattern.len())
        .take_while(|n| *n == 0 || pattern.chars().nth(n - 1) != Some('#'))
        .map(|n| (n, 1))
        .collect()
}

fn is_valid_group(substring: &str, group_size: usize) -> bool {
    (0..group_size).all(|n| matches!(substring.chars().nth(n), Some('#') | Some('?')))
        && substring.chars().nth(group_size) != Some('#')
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
