pub fn solve(input: &str) -> usize {
    let galaxies = get_galaxies(input);

    (0..galaxies.len() - 1)
        .flat_map(|n| {
            let mut it = galaxies.iter().skip(n).copied();
            let galaxy = it.next().unwrap();
            it.map(move |other| galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1))
        })
        .sum()
}

fn get_galaxies(input: &str) -> Vec<(usize, usize)> {
    let map = input.lines().collect::<Vec<_>>();
    let blank_rows = find_blank_rows(&map);
    let blank_cols = find_blank_cols(&map);

    map.into_iter()
        .enumerate()
        .flat_map(|(n, line)| {
            let adjustment = blank_rows.iter().filter(|m| **m < n).count();
            let y = n + adjustment;
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(n, _)| {
                    let adjustment = blank_cols.iter().filter(|m| **m < n).count();
                    n + adjustment
                })
                .map(move |x| (x, y))
        })
        .collect()
}

fn find_blank_rows(map: &[&str]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .map(|(n, _)| n)
        .collect()
}

fn find_blank_cols(map: &[&str]) -> Vec<usize> {
    let len = map.first().unwrap().len();
    map.iter()
        .fold(vec![false; len], |mut acc, line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .for_each(|(n, _)| acc[n] = true);
            acc
        })
        .into_iter()
        .enumerate()
        .filter(|(_, x)| !x)
        .map(|(n, _)| n)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 374);
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
