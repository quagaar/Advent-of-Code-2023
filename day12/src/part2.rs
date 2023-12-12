use rayon::prelude::*;
use std::{collections::HashMap, iter::from_fn, usize};

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
        .fold(initial_state(), |acc, group_size| {
            acc.into_iter()
                .flat_map(|(start_pos, count)| {
                    find_damaged_groups(&pattern, group_size, start_pos, count)
                })
                .fold(HashMap::new(), |mut acc, (pos, count)| {
                    acc.entry(pos)
                        .and_modify(|total| *total += count)
                        .or_insert(count);
                    acc
                })
        })
        .into_iter()
        .filter(|(pos, _)| pattern[*pos..].find('#').is_none())
        .map(|(_, count)| count)
        .sum()
}

fn initial_state() -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    map.insert(0, 1);
    map
}

fn find_damaged_groups(
    pattern: &str,
    group_size: usize,
    start_pos: usize,
    count: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut pos = start_pos;
    from_fn(move || loop {
        match pattern.chars().nth(pos) {
            Some('#') => {
                if let Some(after) = find_after_pos(pattern, pos, group_size) {
                    pos = pattern.len();
                    break Some((after, count));
                } else {
                    break None;
                }
            }
            Some('?') => {
                if let Some(after) = find_after_pos(pattern, pos, group_size) {
                    pos += 1;
                    break Some((after, count));
                } else {
                    pos += 1;
                }
            }
            None => break None,
            _ => pos += 1,
        }
    })
}

fn find_after_pos(pattern: &str, start_pos: usize, group_size: usize) -> Option<usize> {
    if (1..group_size).all(|n| matches!(pattern.chars().nth(start_pos + n), Some('#') | Some('?')))
    {
        match pattern.chars().nth(start_pos + group_size) {
            Some('?') | Some('.') => Some(start_pos + group_size + 1),
            None => Some(start_pos + group_size),
            _ => None,
        }
    } else {
        None
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
        assert_eq!(result, 525152);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 1909291258644);
    }
}
