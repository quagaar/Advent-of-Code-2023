use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day07::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day07 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });

    c.bench_function("solve day07 part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
