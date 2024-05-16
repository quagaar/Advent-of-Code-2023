pub fn solve(input: &str) -> u32 {
    input
        .trim_end()
        .split(',')
        .map(|s| {
            s.as_bytes()
                .iter()
                .fold(0, |acc, c| ((acc + *c as u32) * 17) % 256)
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
        assert_eq!(result, 1320);
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
