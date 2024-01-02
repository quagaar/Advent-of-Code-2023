use itertools::Itertools;
use num::bigint::ToBigInt;
use num::BigInt;
use num::ToPrimitive;
use num_traits::Zero;

pub fn solve(input: &str) -> i64 {
    let hailstones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();

    let xy = solve_equations(
        4,
        &mut hailstones.iter().tuple_combinations().map(|(h1, h2)| {
            vec![
                (h2.vy - h1.vy).to_bigint().unwrap(),
                (h1.py - h2.py).to_bigint().unwrap(),
                (h1.vx - h2.vx).to_bigint().unwrap(),
                (h2.px - h1.px).to_bigint().unwrap(),
                (h2.px * h2.vy - h2.py * h2.vx - h1.px * h1.vy + h1.py * h1.vx)
                    .to_bigint()
                    .unwrap(),
            ]
        }),
    );

    let px = &xy[0];
    let vx = &xy[1];
    let py = &xy[2];
    let _vy = &xy[3];

    let xz = solve_equations(
        4,
        &mut hailstones.iter().tuple_combinations().map(|(h1, h2)| {
            vec![
                (h2.vz - h1.vz).to_bigint().unwrap(),
                (h1.pz - h2.pz).to_bigint().unwrap(),
                (h1.vx - h2.vx).to_bigint().unwrap(),
                (h2.px - h1.px).to_bigint().unwrap(),
                (h2.px * h2.vz - h2.pz * h2.vx - h1.px * h1.vz + h1.pz * h1.vx)
                    .to_bigint()
                    .unwrap(),
            ]
        }),
    );

    debug_assert!(xz[0] == *px);
    debug_assert!(xz[1] == *vx);
    let pz = &xz[2];
    let _vz = &xz[3];

    (px + py + pz).to_i64().unwrap()
}

fn solve_equations(unknowns: usize, eqs: &mut dyn Iterator<Item = Vec<BigInt>>) -> Vec<BigInt> {
    // Skip any rows with a zero coefficient value (they would break the algorithm)
    let mut eqs = eqs
        .filter(|x| x.iter().take(unknowns).all(|x| *x != Zero::zero()))
        .peekable();
    if unknowns == 1 {
        // Base case when only one unknown remains
        let x = eqs.next().unwrap();
        debug_assert!(x.len() == 2);
        debug_assert!(&x[1] % &x[0] == Zero::zero());
        vec![&x[1] / &x[0]]
    } else {
        // Copy one row to use later
        let first = eqs.peek().unwrap().clone();
        // Factor out the first unknown from each row
        let mut next = eqs.tuple_windows().map(|(a, b)| {
            let ma = b[0].clone();
            let mb = a[0].clone();
            a.into_iter()
                .zip(b)
                .skip(1)
                .map(|(a, b)| a * &ma - b * &mb)
                .collect()
        });
        // Solve for the remaining unknowns
        let mut result = solve_equations(unknowns - 1, &mut next);
        // Solve for the first unknown
        let x = first.last().unwrap()
            - first[1..]
                .iter()
                .zip(result.iter())
                .map(|(a, b)| a * b)
                .sum::<BigInt>();
        debug_assert!(&x % &first[0] == Zero::zero());
        result.insert(0, x / &first[0]);
        result
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hailstone {
    fn parse(line: &str) -> Self {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let mut position = position.split(',').map(str::trim).map(str::parse);
        let px = position.next().unwrap().unwrap();
        let py = position.next().unwrap().unwrap();
        let pz = position.next().unwrap().unwrap();
        let mut velocity = velocity.split(',').map(str::trim).map(str::parse);
        let vx = velocity.next().unwrap().unwrap();
        let vy = velocity.next().unwrap().unwrap();
        let vz = velocity.next().unwrap().unwrap();
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
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
        let result = solve(EXAMPLE);
        assert_eq!(result, 47);
    }

    #[test]
    fn result() {
        let result = solve(INPUT);
        assert_eq!(result, 546494494317645);
    }
}
