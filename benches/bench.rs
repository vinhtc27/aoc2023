use aoc2023::days::day3;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    c.bench_function("Day 3 - Part 1 - Solution 1", |b| {
        b.iter(|| day3::part1_sol1(black_box(include_str!("../input/day3/real1.txt"))))
    });
    c.bench_function("Day 3 - Part 2 - Solution 1", |b| {
        b.iter(|| day3::part2_sol1(black_box(include_str!("../input/day3/real2.txt"))))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
