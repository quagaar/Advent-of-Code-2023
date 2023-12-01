fn first_and_last(digits: impl Iterator<Item = char>) -> Option<(char, char)> {
    digits.fold(None, |acc, ch| {
        if let Some((first, _last)) = acc {
            Some((first, ch))
        } else {
            Some((ch, ch))
        }
    })
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit(line: &str) -> Option<usize> {
    let mut result = line
        .chars()
        .enumerate()
        .filter(|(_, b)| b.is_ascii_digit())
        .map(|(pos, ch)| (pos, (ch as usize - '0' as usize)))
        .next();
    for (digit, word) in DIGITS.iter().enumerate() {
        if let Some(pos) = line.find(word) {
            match result {
                None => result = Some((pos, digit)),
                Some((x, _)) if pos < x => result = Some((pos, digit)),
                _ => (),
            }
        }
    }
    result.map(|(_, x)| x)
}

fn last_digit(line: &str) -> Option<usize> {
    let mut result = line
        .chars()
        .enumerate()
        .filter(|(_, b)| b.is_ascii_digit())
        .map(|(pos, ch)| (pos, (ch as usize - '0' as usize)))
        .last();
    for (digit, word) in DIGITS.iter().enumerate() {
        if let Some((pos, _)) = line.match_indices(word).last() {
            match result {
                None => result = Some((pos, digit)),
                Some((x, _)) if pos > x => result = Some((pos, digit)),
                _ => (),
            }
        }
    }
    result.map(|(_, x)| x)
}

pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| first_and_last(line.chars().filter(|ch| ch.is_ascii_digit())))
        .map(|(first, last)| 10 * (first as usize - '0' as usize) + (last as usize - '0' as usize))
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            if let (Some(first), Some(last)) = (first_digit(line), last_digit(line)) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .sum()
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const EXAMPLE2: &str = include_str!("../example2.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 142);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 55108);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE2);
        assert_eq!(result, 281);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 56324);
    }
}
