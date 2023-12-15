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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 1320);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 513158);
    }
}
