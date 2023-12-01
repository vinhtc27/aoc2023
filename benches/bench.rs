use aoc2023::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    c.bench_function("day 01", |b| {
        b.iter(|| day01::run(black_box(include_str!("../input/real/01.txt")), false))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
