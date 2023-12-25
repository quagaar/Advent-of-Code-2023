pub fn solve(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    #[ignore = "not done yet"]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 42);
    }

    #[test]
    #[ignore = "not done yet"]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 42);
    }
}
