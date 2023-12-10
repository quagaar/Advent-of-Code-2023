use bencher::setup_benches;
use criterion::{criterion_group, criterion_main, Criterion};
use day10::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    setup_benches("day10", &part1::solve, &part2::solve, INPUT, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
