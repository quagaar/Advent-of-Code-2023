use bencher::setup_benches;
use criterion::{criterion_group, criterion_main, Criterion};
use day03::{solve_part1, solve_part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    setup_benches("day03", &solve_part1, &solve_part2, INPUT, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
