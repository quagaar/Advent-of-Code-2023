use rayon::prelude::*;

pub fn solve(input: &str) -> usize {
    input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .filter(|(_, rounds)| is_possible_game(rounds))
        .map(|(game, _)| game.split_once(' ').unwrap().1.parse::<usize>().unwrap())
        .sum()
}

fn is_possible_game(rounds: &str) -> bool {
    rounds.split("; ").all(|round| {
        round.split(", ").all(|cube| {
            let (number, colour) = cube.split_once(' ').unwrap();
            let number: usize = number.parse().unwrap();
            match colour {
                "red" => number <= 12,
                "green" => number <= 13,
                "blue" => number <= 14,
                _ => false,
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 8);
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
