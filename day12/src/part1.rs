use std::iter::from_fn;

pub fn solve(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (pattern, counts) = line.split_once(' ').unwrap();

    counts
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .fold(initial_states(pattern), |acc, count| {
            Box::new(
                acc.filter(move |s| has_count_hashes(s, count))
                    .map(move |s| &s[count..])
                    .flat_map(after_hashes_states),
            )
        })
        .filter(|s| s.is_empty())
        .count()
}

fn initial_states<'a>(pattern: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(
        (0..pattern.len())
            .take_while(|n| *n == 0 || pattern.chars().nth(n - 1) != Some('#'))
            .map(|n| &pattern[n..]),
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
        assert_eq!(result, 21);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 7260);
    }
}
