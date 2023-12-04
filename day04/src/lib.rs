pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(Card::try_parse)
        .map(|card| {
            let n = card.count_matches();
            if n > 0 {
                1 << (n - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let matches = input
        .lines()
        .filter_map(Card::try_parse)
        .map(Card::count_matches)
        .collect::<Vec<_>>();
    let mut cards = matches.iter().map(|_| 1).collect::<Vec<usize>>();
    matches.into_iter().enumerate().for_each(|(n, matches)| {
        if matches > 0 {
            let copies = cards[n];
            (n + 1..=n + matches).for_each(|m| {
                if let Some(s) = cards.get_mut(m) {
                    *s += copies;
                }
            })
        }
    });
    cards.into_iter().sum()
}

struct Card<'a> {
    winning: &'a str,
    numbers: &'a str,
}

impl<'a> Card<'a> {
    fn try_parse(line: &'a str) -> Option<Self> {
        let (_, rhs) = line.split_once(": ")?;
        let (winning, numbers) = rhs.split_once(" | ")?;
        Some(Self { winning, numbers })
    }

    fn count_matches(self) -> usize {
        self.numbers
            .split_ascii_whitespace()
            .filter(|s| self.winning.split_ascii_whitespace().any(|n| n == *s))
            .count()
    }
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 23678);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 30);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 15455663);
    }
}
