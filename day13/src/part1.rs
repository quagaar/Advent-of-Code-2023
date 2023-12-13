use std::usize;

pub fn solve(input: &str) -> usize {
    input.split("\n\n").map(process_pattern).sum()
}

fn process_pattern(pattern: &str) -> usize {
    let map = pattern.lines().map(str::as_bytes).collect::<Vec<_>>();

    let vertical_lines = map
        .iter()
        .fold((1..map[0].len()).collect::<Vec<_>>(), |acc, line| {
            acc.into_iter()
                .filter(|pos| is_vertical_reflection(line, *pos))
                .collect()
        });

    let horizontal_lines = (1..map.len())
        .filter(|pos| is_horizontal_reflection(&map, *pos))
        .collect::<Vec<_>>();

    debug_assert!(vertical_lines.len() | horizontal_lines.len() == 1);

    vertical_lines.into_iter().sum::<usize>() + horizontal_lines.into_iter().sum::<usize>() * 100
}

fn is_vertical_reflection(line: &[u8], pos: usize) -> bool {
    if line.len() / 2 < pos {
        (pos..line.len()).all(|n| line[n] == line[pos * 2 - n - 1])
    } else {
        (0..pos).all(|n| line[n] == line[pos * 2 - n - 1])
    }
}

fn is_horizontal_reflection(map: &[&[u8]], pos: usize) -> bool {
    if map.len() / 2 < pos {
        (pos..map.len()).all(|n| map[n] == map[pos * 2 - n - 1])
    } else {
        (0..pos).all(|n| map[n] == map[pos * 2 - n - 1])
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
        assert_eq!(result, 405);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 27502);
    }
}
