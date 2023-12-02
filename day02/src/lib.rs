pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(Game::try_parse)
        .filter(is_possible_game)
        .map(|game| game.id)
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(Game::try_parse)
        .map(get_cube_power)
        .sum()
}
struct Game<'a> {
    id: usize,
    rounds: Vec<Vec<(usize, &'a str)>>,
}

impl<'a> Game<'a> {
    fn try_parse(line: &'a str) -> Option<Self> {
        let (game, rounds) = line.split_once(": ")?;
        let id = game.split_once(' ')?.1.parse().ok()?;
        let rounds = rounds
            .split("; ")
            .map(|round| {
                round
                    .split(", ")
                    .filter_map(|cube| {
                        let (number, colour) = cube.split_once(' ')?;
                        let number = number.parse().ok()?;
                        Some((number, colour))
                    })
                    .collect()
            })
            .collect();
        Some(Game { id, rounds })
    }
}

fn is_possible_game(game: &Game) -> bool {
    game.rounds.iter().all(|round| {
        round.iter().all(|(number, colour)| match *colour {
            "red" => *number <= 12,
            "green" => *number <= 13,
            "blue" => *number <= 14,
            _ => false,
        })
    })
}

fn get_cube_power(game: Game) -> usize {
    let (red, green, blue) = game.rounds.into_iter().fold((0, 0, 0), |mut acc, round| {
        round.into_iter().for_each(|(count, colour)| match colour {
            "red" => acc.0 = acc.0.max(count),
            "green" => acc.1 = acc.1.max(count),
            "blue" => acc.2 = acc.2.max(count),
            _ => (),
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
