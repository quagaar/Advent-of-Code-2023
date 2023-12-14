pub fn solve(input: &str) -> usize {
    let map = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    (0..map[0].len())
        .map(|n| {
            let mut total = 0;
            let mut cost = map.len();
            for (m, row) in map.iter().enumerate() {
                match row[n] {
                    b'O' => {
                        total += cost;
                        cost -= 1;
                    }
                    b'#' => {
                        cost = map.len() - m - 1;
                    }
                    _ => (),
                }
            }
            total
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
        assert_eq!(result, 136);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 106648);
    }
}
