use rayon::prelude::*;

pub fn solve_part2(input: &str) -> usize {
    input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(_, rounds)| get_cube_power(rounds))
        .sum()
}

fn get_cube_power(rounds: &str) -> usize {
    let (red, green, blue) =
        rounds
            .split("; ")
            .fold((0, 0, 0), |(mut red, mut green, mut blue), round| {
                round.split(", ").for_each(|cube| {
                    let (number, colour) = cube.split_once(' ').unwrap();
                    let number: usize = number.parse().unwrap();
                    match colour {
                        "red" => red = red.max(number),
                        "green" => green = green.max(number),
                        "blue" => blue = blue.max(number),
                        _ => (),
                    }
                });
                (red, green, blue)
            });
    red * green * blue
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 2286);
    }

    #[test]
    fn result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 56580);
    }
}
