use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day23::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day23 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });
    let mut group = c.benchmark_group("solve day23 part2");
    group.sample_size(10);
    group.bench_function("solve day23 part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
