use aoc2023::days::{day1, day2, day3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    c.bench_function("Day 1 - Part 1 - Solution 1", |b| {
        b.iter(|| day1::part1_sol1(black_box(include_str!("../input/day1/real1.txt"))))
    });
    c.bench_function("Day 1 - Part 1 - Solution 2", |b| {
        b.iter(|| day1::part1_sol2(black_box(include_str!("../input/day1/real1.txt"))))
    });
    c.bench_function("Day 1 - Part 1 - Solution 3", |b| {
        b.iter(|| day1::part1_sol3(black_box(include_str!("../input/day1/real1.txt"))))
    });
    c.bench_function("Day 1 - Part 1 - Solution 4", |b| {
        b.iter(|| day1::part1_sol4(black_box(include_str!("../input/day1/real1.txt"))))
    });
    c.bench_function("Day 1 - Part 1 - Solution 5", |b| {
        b.iter(|| day1::part1_sol5(black_box(include_str!("../input/day1/real1.txt"))))
    });
    c.bench_function("Day 1 - Part 2 - Solution 1", |b| {
        b.iter(|| day1::part2_sol1(black_box(include_str!("../input/day1/real2.txt"))))
    });
    c.bench_function("Day 1 - Part 2 - Solution 2", |b| {
        b.iter(|| day1::part2_sol2(black_box(include_str!("../input/day1/real2.txt"))))
    });
    c.bench_function("Day 1 - Part 2 - Solution 3", |b| {
        b.iter(|| day1::part2_sol3(black_box(include_str!("../input/day1/real2.txt"))))
    });
    c.bench_function("Day 2 - Part 1 - Solution 1", |b| {
        b.iter(|| day2::part1_sol1(black_box(include_str!("../input/day2/real1.txt"))))
    });
    c.bench_function("Day 2 - Part 2 - Solution 1", |b| {
        b.iter(|| day2::part2_sol1(black_box(include_str!("../input/day2/real2.txt"))))
    });
    c.bench_function("Day 3 - Part 1 - Solution 1", |b| {
        b.iter(|| day3::part1_sol1(black_box(include_str!("../input/day3/real1.txt"))))
    });
    c.bench_function("Day 3 - Part 2 - Solution 1", |b| {
        b.iter(|| day3::part2_sol1(black_box(include_str!("../input/day3/real2.txt"))))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
