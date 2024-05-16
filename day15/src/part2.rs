pub fn solve(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .fold(vec![vec![]; 256], |mut acc, s| {
            if let Some(label) = s.strip_suffix('-') {
                let box_no = hash(label);
                acc[box_no].retain(|(l, _)| *l != label);
            } else {
                let (label, focal_length) = s.split_once('=').unwrap();
                let box_no = hash(label);
                let focal_length = focal_length.parse::<u8>().unwrap();
                if let Some((_, x)) = acc[box_no].iter_mut().find(|(l, _)| *l == label) {
                    *x = focal_length;
                } else {
                    acc[box_no].push((label, focal_length));
                }
            }
            acc
        })
        .into_iter()
        .enumerate()
        .map(|(box_no, lenses)| {
            (box_no + 1)
                * lenses
                    .into_iter()
                    .enumerate()
                    .map(|(n, (_, focal_length))| (n + 1) * focal_length as usize)
                    .sum::<usize>()
        })
        .sum()
}

fn hash(substring: &str) -> usize {
    substring
        .as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 145);
    }

    #[cfg(input_txt)]
    #[cfg(part2_txt)]
    #[test]
    fn result() {
        let expected = include_str!("../part2.txt").trim().parse().unwrap();
        let result = solve(super::super::INPUT);
        assert_eq!(result, expected);
    }
}
