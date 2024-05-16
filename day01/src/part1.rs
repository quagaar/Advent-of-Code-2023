use rayon::prelude::*;

pub fn solve(input: &str) -> u32 {
    input
        .par_lines()
        .filter_map(|line| {
            if let (Some(first), Some(last)) = (
                line.chars().find_map(|c| c.to_digit(10)),
                line.chars().rev().find_map(|c| c.to_digit(10)),
            ) {
                Some(10 * first + last)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 142);
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
