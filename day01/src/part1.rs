use rayon::prelude::*;

pub fn solve_part1(input: &str) -> u32 {
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 142);
    }

    #[test]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 55108);
    }
}
