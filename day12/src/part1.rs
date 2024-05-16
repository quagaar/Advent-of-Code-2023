use rayon::prelude::*;
use std::{collections::BTreeMap, iter::from_fn};

pub fn solve(input: &str) -> usize {
    input.par_lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (pattern, group_sizes) = line.split_once(' ').unwrap();
    let pattern = pattern.as_bytes();

    group_sizes
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .fold(initial_states(pattern), |acc, group_size| {
            acc.into_iter()
                .filter(|(start_pos, _)| is_valid_group(&pattern[*start_pos..], group_size))
                .fold(BTreeMap::new(), |mut acc, (start_pos, count)| {
                    let pos = start_pos + group_size;
                    after_hashes_positions(&pattern[pos..]).for_each(|offset| {
                        acc.entry(pos + offset)
                            .and_modify(|total| *total += count)
                            .or_insert(count);
                    });
                    acc
                })
        })
        .get(&pattern.len())
        .copied()
        .unwrap_or(0)
}

/// Get the initial states to consider for the start of the first group.
fn initial_states(pattern: &[u8]) -> BTreeMap<usize, usize> {
    if let Some(pos) = pattern.iter().position(|&c| c == b'#') {
        (0..=pos).map(|n| (n, 1)).collect()
    } else {
        (0..=pattern.len()).map(|n| (n, 1)).collect()
    }
}

/// Check if a valid group of given size can fit at the start of the substring.
fn is_valid_group(substring: &[u8], group_size: usize) -> bool {
    let mut chars = substring.iter().copied();
    // first group_size characters are either '#' or '?'
    (0..group_size).all(|_| matches!(chars.next(), Some(b'#') | Some(b'?')))
            // and the next character is not '#'
            && chars.next() != Some(b'#')
}

/// Get all positions after a group where the next group could start, and
/// the end position of the pattern if there is no # characters before it.
fn after_hashes_positions(after_pattern: &[u8]) -> impl Iterator<Item = usize> + '_ {
    let mut chars = after_pattern.iter().copied();
    let mut prev = None;
    let mut pos = 0;
    from_fn(move || {
        if prev == Some(b'#') {
            None
        } else {
            let next = (pos, chars.next());
            prev = next.1.or(Some(b'#'));
            pos += 1;
            Some(next)
        }
    })
    .filter(|x| !matches!(x, (0, Some(_)) | (_, Some(b'.'))))
    .map(|(offset, _)| offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 21);
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
