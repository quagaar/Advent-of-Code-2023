use rayon::prelude::*;

pub fn solve_part1(input: &str) -> usize {
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 2727);
    }
}
