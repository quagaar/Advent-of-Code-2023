use std::{iter::from_fn, ops::Range};

pub fn solve_part1(input: &str) -> usize {
    let seeds = get_seeds(input);
    let maps = get_maps(input);
    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, m| m.convert(acc)))
        .min()
        .unwrap()
}

fn get_seeds(input: &str) -> Vec<usize> {
    let (_, seeds) = input.lines().next().unwrap().split_once(": ").unwrap();
    seeds
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_maps(input: &str) -> Vec<Map<'_>> {
    let mut lines = input.lines().skip(2);
    from_fn(move || Map::try_parse(&mut lines)).collect()
}

struct Map<'a> {
    _name: &'a str,
    ranges: Vec<MapRange>,
}

impl<'a> Map<'a> {
    fn try_parse(lines: &mut impl Iterator<Item = &'a str>) -> Option<Self> {
        let (name, _) = lines.next()?.split_once(" map:")?;
        let ranges = lines.map_while(MapRange::try_parse).collect();
        Some(Map {
            _name: name,
            ranges,
        })
    }

    fn convert(&self, value: usize) -> usize {
        for range in self.ranges.iter() {
            if range.source.contains(&value) {
                return value - range.source.start + range.destination.start;
            }
        }
        value
    }
}

struct MapRange {
    destination: Range<usize>,
    source: Range<usize>,
}

impl MapRange {
    fn try_parse(line: &str) -> Option<MapRange> {
        let (first, rest) = line.split_once(' ')?;
        let (second, third) = rest.split_once(' ')?;
        let destination: usize = first.parse().ok()?;
        let source: usize = second.parse().ok()?;
        let length: usize = third.parse().ok()?;
        Some(MapRange {
            destination: destination..destination + length,
            source: source..source + length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 525792406);
    }
}
