use rayon::prelude::*;

pub fn solve_part1(input: &str) -> usize {
    input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .filter(|(_, rounds)| is_possible_game(rounds))
        .map(|(game, _)| game.split_once(' ').unwrap().1.parse::<usize>().unwrap())
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(_, rounds)| get_cube_power(rounds))
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

fn get_cube_power(rounds: &str) -> usize {
    let (red, green, blue) = rounds.split("; ").fold((0, 0, 0), |mut acc, round| {
        round.split(", ").for_each(|cube| {
            let (number, colour) = cube.split_once(' ').unwrap();
            let number: usize = number.parse().unwrap();
            match colour {
                "red" => acc.0 = acc.0.max(number),
                "green" => acc.1 = acc.1.max(number),
                "blue" => acc.2 = acc.2.max(number),
                _ => (),
            }
        });
        acc
    });
    red * green * blue
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 2727);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 2286);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 56580);
    }
}
