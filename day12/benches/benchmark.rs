use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day12::{part1, part2, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve day12 part1", |b| {
        b.iter(|| part1::solve(black_box(INPUT)));
    });

    c.bench_function("solve day12 part2", |b| {
        b.iter(|| part2::solve(black_box(INPUT)));
    });

    let mut group = c.benchmark_group("process_line day12");
    group.sample_size(1000);
    group.bench_function("part1", |b| {
        let mut lines = INPUT.lines().cycle();
        b.iter_batched(
            move || lines.next().unwrap(),
            part1::process_line,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("part2", |b| {
        let mut lines = INPUT.lines().cycle();
        b.iter_batched(
            move || lines.next().unwrap(),
            part2::process_line,
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
