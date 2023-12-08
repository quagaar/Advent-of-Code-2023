use num::Integer;
use rayon::prelude::*;
use std::collections::{hash_map::Entry, HashMap, HashSet};

pub fn solve_part2(input: &str) -> usize {
    let directions = input.lines().next().unwrap();
    let map = input
        .lines()
        .skip(2)
        .filter_map(read_map_line)
        .collect::<HashMap<_, _>>();

    let start_locations = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    let repeat_info = start_locations
        .into_par_iter()
        .map(|start| get_repeat_info(start, directions, &map))
        .collect::<Vec<_>>();

    let period_lcm = repeat_info
        .iter()
        .map(|info| info.period)
        .reduce(|lhs, rhs| lhs.lcm(&rhs))
        .unwrap();

    repeat_info
        .iter()
        .map(|info| {
            info.targets
                .keys()
                .map(|(x, _)| x)
                .copied()
                .collect::<HashSet<_>>()
        })
        .reduce(|lhs, rhs| lhs.intersection(&rhs).copied().collect())
        .expect("No common targets found")
        .into_iter()
        .map(|target_offset| {
            let period = repeat_info[0].period;
            let start = repeat_info[0]
                .targets
                .iter()
                .find(|((offset, _), _)| target_offset == *offset)
                .map(|(_, start)| *start)
                .unwrap();
            period_lcm - period + start + 1
        })
        .min()
        .unwrap()
}

fn read_map_line(line: &str) -> Option<(&str, (&str, &str))> {
    let (loc, next) = line.split_once(" = ")?;
    let (left, right) = next.trim_matches(['(', ')'].as_ref()).split_once(", ")?;
    Some((loc, (left, right)))
}

struct RepeatInfo<'a> {
    period: usize,
    targets: HashMap<(usize, &'a str), usize>,
}

fn get_repeat_info<'a>(
    start: &'a str,
    directions: &str,
    map: &HashMap<&'a str, (&'a str, &'a str)>,
) -> RepeatInfo<'a> {
    let mut visited = HashMap::new();
    let mut location = start;
    for (steps, (offset, direction)) in directions.chars().enumerate().cycle().enumerate() {
        let next = map.get(location).expect("Unknown location");
        match direction {
            'L' => location = next.0,
            'R' => location = next.1,
            other => panic!("Invalid direction: {}", other),
        }
        match visited.entry((offset, location)) {
            Entry::Vacant(entry) => {
                entry.insert(steps);
            }
            Entry::Occupied(entry) => {
                let period = steps - entry.get();
                let targets = visited
                    .into_iter()
                    .filter(|((_, location), _)| location.ends_with('Z'))
                    .collect();
                return RepeatInfo { period, targets };
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE3);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 23977527174353);
    }
}
