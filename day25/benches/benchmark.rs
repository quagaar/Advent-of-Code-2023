use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day25::{part1, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day25 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
