use criterion::{black_box, Criterion};

pub fn setup_benches<R1, R2>(
    day: &'static str,
    solve_part1: &'static impl Fn(&str) -> R1,
    solve_part2: &'static impl Fn(&str) -> R2,
    input: &'static str,
    c: &mut Criterion,
) {
    let id = format!("solve {} part1", day);
    c.bench_function(id.as_str(), |b| {
        b.iter(|| solve_part1(black_box(input)));
    });

    let id = format!("solve {} part2", day);
    c.bench_function(id.as_str(), |b| {
        b.iter(|| solve_part2(black_box(input)));
    });
}
