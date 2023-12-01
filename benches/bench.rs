use aoc2023::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    c.bench_function("day 01 - part1", |b| {
        b.iter(|| day01::part1(black_box(include_str!("../input/day1/real1.txt")), false))
    });
    c.bench_function("day 01 - part2", |b| {
        b.iter(|| day01::part2(black_box(include_str!("../input/day1/real2.txt")), false))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
