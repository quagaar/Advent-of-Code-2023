use rayon::prelude::*;

pub fn solve_part1(input: &str) -> usize {
    input
        .par_lines()
        .filter_map(Card::try_parse)
        .map(|card| {
            if let Some(n) = card.count_matches().checked_sub(1) {
                1 << n
            } else {
                0
            }
        })
        .sum()
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
        self.winning
            .split_ascii_whitespace()
            .filter(|s| self.numbers.split_ascii_whitespace().any(|n| n == *s))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 23678);
    }
}
