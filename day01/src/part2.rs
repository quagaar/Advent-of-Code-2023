use rayon::prelude::*;

pub fn solve_part2(input: &str) -> u32 {
    input
        .par_lines()
        .filter_map(|line| {
            if let (Some(first), Some(last)) = (
                (0..line.len()).find_map(|x| lookup_digit(&line[x..])),
                (0..line.len()).rev().find_map(|x| lookup_digit(&line[x..])),
            ) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .sum()
}

const LUT: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn lookup_digit(s: &str) -> Option<u32> {
    LUT.iter().find_map(|(word, digit)| {
        if s.starts_with(word) {
            Some(*digit)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 281);
    }

    #[test]
    fn result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 56324);
    }
}
