use itertools::Itertools;

pub fn solve(input: &str) -> usize {
    count_collisions(input, 200000000000000.0, 400000000000000.0)
}

fn count_collisions(input: &str, min: f64, max: f64) -> usize {
    input
        .lines()
        .map(Hailstone::parse)
        .map(IntersectionCalculator::new)
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            a.intersection(&b)
                .filter(|intersection| a.hailstone.is_in_future(*intersection))
                .filter(|intersection| b.hailstone.is_in_future(*intersection))
        })
        .filter(|(x, y)| *x >= min && *x <= max && *y >= min && *y <= max)
        .count()
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Hailstone {
    fn parse(line: &str) -> Self {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let mut position = position.split(',').map(str::trim).map(str::parse);
        let px = position.next().unwrap().unwrap();
        let py = position.next().unwrap().unwrap();
        let mut velocity = velocity.split(',').map(str::trim).map(str::parse);
        let vx = velocity.next().unwrap().unwrap();
        let vy = velocity.next().unwrap().unwrap();
        Self { px, py, vx, vy }
    }

    fn is_in_future(&self, intersection: (f64, f64)) -> bool {
        let (x, _y) = intersection;
        if self.vx < 0 {
            (self.px as f64) > x
        } else {
            (self.px as f64) < x
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct IntersectionCalculator {
    hailstone: Hailstone,
    // (x - x') = (px - (px + vx)) = -vx
    a: f64,
    // (y - y') = (py - (py + vy)) = -vy
    b: f64,
    // (x*y' - y*x') = px*vy - py*vx
    c: f64,
}

impl IntersectionCalculator {
    fn new(hailstone: Hailstone) -> Self {
        let a = hailstone.vx as f64 * -1.0;
        let b = hailstone.vy as f64 * -1.0;
        let c =
            hailstone.px as f64 * hailstone.vy as f64 - hailstone.py as f64 * hailstone.vx as f64;
        Self { hailstone, a, b, c }
    }

    fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let det = self.a * other.b - other.a * self.b;
        if det == 0.0 {
            return None;
        }
        let x = (self.c * other.a - self.a * other.c) / det;
        let y = (self.c * other.b - self.b * other.c) / det;
        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = count_collisions(EXAMPLE, 7.0, 27.0);
        assert_eq!(result, 2);
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
