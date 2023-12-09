use itertools::Itertools;
use rayon::prelude::*;
use std::{iter::from_fn, ops::Range};

pub fn solve_part2(input: &str) -> usize {
    let seed_ranges = get_seed_ranges(input);
    let maps = get_maps(input);
    seed_ranges
        .into_par_iter()
        .map(|seed_range| {
            maps.iter()
                .fold(vec![seed_range], |acc, m| m.convert_ranges(acc))
                .into_iter()
                .map(|r| r.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn get_seed_ranges(input: &str) -> Vec<Range<usize>> {
    let (_, seeds) = input.lines().next().unwrap().split_once(": ").unwrap();
    seeds
        .split_ascii_whitespace()
        .batching(|it| {
            let start: usize = it.next()?.parse().ok()?;
            let len: usize = it.next()?.parse().ok()?;
            Some(start..start + len)
        })
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
                        MaybeTwo::One(r)
                    }
                })
                .collect();
        }
        destination_ranges.append(&mut source_ranges);
        destination_ranges
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

    fn convert_range(
        &self,
        range: &Range<usize>,
    ) -> Option<(Range<usize>, MaybeTwo<Range<usize>>)> {
        if range.end <= self.source.start || range.start >= self.source.end {
            // Range is outside mapping range
            None
        } else if range.start >= self.source.start {
            if range.end <= self.source.end {
                // Range is fully contained in mapping range
                let start = range.start - self.source.start + self.destination.start;
                let end = range.end - self.source.start + self.destination.start;
                Some((start..end, MaybeTwo::None))
            } else {
                // Range overlaps end of mapping range
                let start = range.start - self.source.start + self.destination.start;
                let end = self.destination.end;
                let after = self.source.end..range.end;
                Some((start..end, MaybeTwo::One(after)))
            }
        } else if range.end > self.source.end {
            // Range surrounds mapping range
            let start = self.destination.start;
            let end = self.destination.end;
            let before = range.start..self.source.start;
            let after = self.source.end..range.end;
            Some((start..end, MaybeTwo::Two(before, after)))
        } else {
            // Range overlaps start of mapping range
            let start = self.destination.start;
            let end = range.end - self.source.start + self.destination.start;
            let before = range.start..self.source.start;
            Some((start..end, MaybeTwo::One(before)))
        }
    }
}

enum MaybeTwo<T: Clone> {
    None,
    One(T),
    Two(T, T),
}

impl<T: Clone> IntoIterator for MaybeTwo<T> {
    type Item = T;

    type IntoIter = MaybeTwoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MaybeTwoIterator(self, 0)
    }
}
struct MaybeTwoIterator<T: Clone>(MaybeTwo<T>, i8);

impl<T: Clone> Iterator for MaybeTwoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self(MaybeTwo::One(x), 0) => {
                self.1 += 1;
                Some(x.clone())
            }
            Self(MaybeTwo::Two(x, _), 0) => {
                self.1 += 1;
                Some(x.clone())
            }
            Self(MaybeTwo::Two(_, x), 1) => {
                self.1 += 1;
                Some(x.clone())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 79004094);
    }
}
