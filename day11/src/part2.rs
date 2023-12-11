pub fn solve(input: &str) -> usize {
    get_expanded(input, 1000000 - 1)
}

fn get_expanded(input: &str, expansion: usize) -> usize {
    let galaxies = get_galaxies(input, expansion);

    (0..galaxies.len() - 1)
        .map(|n| {
            let mut it = galaxies.iter().skip(n).copied();
            let galaxy = it.next().unwrap();
            it.map(|other| galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1))
                .sum::<usize>()
        })
        .sum()
}

fn get_galaxies(input: &str, expansion: usize) -> Vec<(usize, usize)> {
    let map = input.lines().collect::<Vec<_>>();
    let blank_rows = find_blank_rows(&map);
    let blank_cols = find_blank_cols(&map);

    map.into_iter()
        .enumerate()
        .flat_map(|(n, line)| {
            let adjustment = blank_rows.iter().filter(|m| **m < n).count();
            let y = n + (adjustment * expansion);
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(n, _)| {
                    let adjustment = blank_cols.iter().filter(|m| **m < n).count();
                    n + (adjustment * expansion)
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_10x() {
        let result = get_expanded(EXAMPLE, 10 - 1);
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_100x() {
        let result = get_expanded(EXAMPLE, 100 - 1);
        assert_eq!(result, 8410);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 382979724122);
    }
}
