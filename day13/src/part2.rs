use std::usize;

pub fn solve(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

fn process_pattern(pattern: &str) -> usize {
    let map = pattern.lines().map(str::as_bytes).collect::<Vec<_>>();

    let vertical_lines = map
        .iter()
        .fold(vec![0; map[0].len()], |mut acc, line| {
            acc.iter_mut().enumerate().skip(1).for_each(|(n, count)| {
                *count += count_vertical_smudges(line, n);
            });
            acc
        })
        .into_iter()
        .enumerate()
        .skip(1)
        .filter(|(_, count)| *count == 1)
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();

    let horizontal_lines = (1..map.len())
        .filter(|pos| count_horizontal_smudges(&map, *pos) == 1)
        .collect::<Vec<_>>();

    debug_assert!(vertical_lines.len() | horizontal_lines.len() == 1);

    vertical_lines.into_iter().sum::<usize>() + horizontal_lines.into_iter().sum::<usize>() * 100
}

fn count_vertical_smudges(line: &[u8], pos: usize) -> usize {
    if line.len() / 2 < pos {
        (pos..line.len())
            .filter(|n| line[*n] != line[pos * 2 - n - 1])
            .count()
    } else {
        (0..pos)
            .filter(|n| line[*n] != line[pos * 2 - n - 1])
            .count()
    }
}

fn count_horizontal_smudges(map: &[&[u8]], pos: usize) -> usize {
    if map.len() / 2 < pos {
        (pos..map.len())
            .map(|n| count_differences(map[n], map[pos * 2 - n - 1]))
            .sum()
    } else {
        (0..pos)
            .map(|n| count_differences(map[n], map[pos * 2 - n - 1]))
            .sum()
    }
}

fn count_differences(lhs: &[u8], rhs: &[u8]) -> usize {
    lhs.iter().zip(rhs.iter()).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 400);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 31947);
    }
}
