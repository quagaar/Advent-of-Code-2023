use itertools::Itertools;
use num::{bigint::ToBigInt, BigInt};
use num::{Integer, ToPrimitive};
use num_traits::{One, Zero};

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
    if unknowns == 1 {
        let x = eqs.next().unwrap();
        debug_assert!(x.len() == 2);
        debug_assert!(&x[1] % &x[0] == Zero::zero());
        vec![&x[1] / &x[0]]
    } else {
        let mut eqs = eqs.filter(|x| x[0] != Zero::zero()).peekable();
        let first = eqs.peek().unwrap().clone();
        let mut next = eqs
            .tuple_windows()
            .map(|(a, b)| {
                let gcd = a[0].gcd(&b[0]);
                let ma = &b[0] / &gcd;
                let mb = -&a[0] / gcd;
                let mut res = vec![Zero::zero(); unknowns];
                for i in 0..unknowns {
                    res[i] = &a[i + 1] * &ma + &b[i + 1] * &mb;
                }
                let gcd = res
                    .iter()
                    .fold(None, |acc, x| {
                        if let Some(y) = acc {
                            Some(x.gcd(&y))
                        } else {
                            Some(x.clone())
                        }
                    })
                    .unwrap();
                if gcd > One::one() {
                    res.iter_mut().for_each(|x| *x /= &gcd);
                }
                res
            })
            .filter(|x| x.iter().take(unknowns - 1).all(|x| *x != Zero::zero()));
        let mut result = solve_equations(unknowns - 1, &mut next);
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
