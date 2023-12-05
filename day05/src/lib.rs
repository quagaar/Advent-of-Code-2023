use itertools::Itertools;
use std::{iter::from_fn, ops::Range, str::Lines};

pub fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = get_seeds(lines.next().unwrap());
    lines.next();
    let maps = get_maps(lines).collect::<Vec<_>>();
    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, m| m.convert(acc)))
        .min()
        .unwrap()
}

pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    let seeds = get_seed_ranges(lines.next().unwrap());
    lines.next();
    let maps = get_maps(lines).collect::<Vec<_>>();
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(vec![seed], |acc, m| m.convert_ranges(acc))
                .into_iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn get_seeds(line: &str) -> Vec<usize> {
    let (_, seeds) = line.split_once(": ").unwrap();
    seeds
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_seed_ranges(line: &str) -> Vec<Range<usize>> {
    let (_, seeds) = line.split_once(": ").unwrap();
    seeds
        .split_ascii_whitespace()
        .batching(|it| {
            let start: usize = it.next()?.parse().ok()?;
            let len: usize = it.next()?.parse().ok()?;
            Some(start..start + len)
        })
        .collect()
}

fn get_maps(mut lines: Lines<'_>) -> impl Iterator<Item = Map<'_>> {
    from_fn(move || Map::try_parse(&mut lines))
}

struct Map<'a> {
    _name: &'a str,
    ranges: Vec<MapRange>,
}

impl<'a> Map<'a> {
    fn try_parse(lines: &mut Lines<'a>) -> Option<Self> {
        let (name, _) = lines.next()?.split_once(" map:")?;
        let ranges = lines.map_while(MapRange::try_parse).collect();
        Some(Map {
            _name: name,
            ranges,
        })
    }

    fn convert(&self, value: usize) -> usize {
        for range in self.ranges.iter() {
            if value >= range.source && value < range.source + range.length {
                return value - range.source + range.destination;
            }
        }
        value
    }

    fn convert_ranges(&self, mut source_ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut destination_ranges = vec![];
        for map_range in self.ranges.iter() {
            source_ranges = source_ranges
                .into_iter()
                .flat_map(|r| {
                    if let Some((dest, remaining)) = map_range.convert_range(&r) {
                        destination_ranges.push(dest);
                        remaining
                    } else {
                        vec![r]
                    }
                })
                .collect();
        }
        destination_ranges.append(&mut source_ranges);
        destination_ranges
    }
}

struct MapRange {
    destination: usize,
    source: usize,
    length: usize,
}

impl MapRange {
    fn try_parse(line: &str) -> Option<MapRange> {
        let (first, rest) = line.split_once(' ')?;
        let (second, third) = rest.split_once(' ')?;
        let destination = first.parse().ok()?;
        let source = second.parse().ok()?;
        let length = third.parse().ok()?;
        Some(MapRange {
            destination,
            source,
            length,
        })
    }

    #[allow(clippy::single_range_in_vec_init)]
    fn convert_range(&self, range: &Range<usize>) -> Option<(Range<usize>, Vec<Range<usize>>)> {
        let source_end = self.source + self.length;
        if range.end <= self.source || range.start >= source_end {
            // Range is outside
            None
        } else if range.start >= self.source {
            if range.end <= source_end {
                // Range is fully contained
                let start = range.start - self.source + self.destination;
                let end = range.end - self.source + self.destination;
                Some((start..end, vec![]))
            } else {
                // Range overlaps end of mapping range
                let start = range.start - self.source + self.destination;
                let end = self.destination + self.length;
                let after = source_end..range.end;
                Some((start..end, vec![after]))
            }
        } else if range.end > source_end {
            // Range surrounds mapping range
            let start = self.destination;
            let end = self.destination + self.length;
            let before = range.start..self.source;
            let after = source_end..range.end;
            Some((start..end, vec![before, after]))
        } else {
            // Range overlaps start of mapping range
            let start = self.destination;
            let end = range.end - self.source + self.destination;
            let before = range.start..self.source;
            Some((start..end, vec![before]))
        }
    }
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 525792406);
    }

    #[test]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 79004094);
    }
}
