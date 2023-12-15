use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .fold(HashMap::new(), |mut acc, s| {
            if let Some(label) = s.strip_suffix('-') {
                let box_no = hash(label);
                acc.entry(box_no)
                    .and_modify(|lenses: &mut Vec<(&str, u8)>| lenses.retain(|(l, _)| *l != label));
            } else {
                let (label, focal_length) = s.split_once('=').unwrap();
                let box_no = hash(label);
                let focal_length = focal_length.parse::<u8>().unwrap();
                acc.entry(box_no)
                    .and_modify(|lenses| {
                        if let Some((_, x)) = lenses.iter_mut().find(|(l, _)| *l == label) {
                            *x = focal_length;
                        } else {
                            lenses.push((label, focal_length));
                        }
                    })
                    .or_insert(vec![(label, focal_length)]);
            }
            acc
        })
        .into_iter()
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
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve(EXAMPLE);
        assert_eq!(result, 145);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 200277);
    }
}
