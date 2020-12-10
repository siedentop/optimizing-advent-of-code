use advent_of_code::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn part1_benchmark(c: &mut Criterion) {
    let data = include_str!("../data/day09");
    let rows: Vec<String> = data.split('\n').map(str::to_owned).collect();

    let numbers = parse_file(rows).unwrap();

    assert_eq!(part1(&numbers, 25).unwrap(), 25918798);
    assert_eq!(part1_b(&numbers, 25).unwrap(), 25918798);
    assert_eq!(part1_c(&numbers, 25).unwrap(), 25918798);
    assert_eq!(part1_d(&numbers, 25).unwrap(), 25918798);
    assert_eq!(part1_e(&numbers, 25).unwrap(), 25918798);

    let mut group = c.benchmark_group("Part 1");

    group.bench_function("impl A", |b| b.iter(|| part1(black_box(&numbers), 25)));
    group.bench_function("impl B", |b| b.iter(|| part1_b(black_box(&numbers), 25)));
    group.bench_function("impl C", |b| b.iter(|| part1_c(black_box(&numbers), 25)));
    group.bench_function("impl D", |b| b.iter(|| part1_d(black_box(&numbers), 25)));
    group.bench_function("impl E", |b| b.iter(|| part1_e(black_box(&numbers), 25)));
    group.finish();
}

pub fn part2_benchmark(c: &mut Criterion) {
    let data = include_str!("../data/day09");
    let rows: Vec<String> = data.split('\n').map(str::to_owned).collect();

    let numbers = parse_file(rows).unwrap();

    assert_eq!(part2(&numbers, 25918798).unwrap(), 3340942);
    assert_eq!(part2_b(&numbers, 25918798).unwrap(), 3340942);

    let mut group = c.benchmark_group("Part 2");

    group.bench_function("impl A", |b| {
        b.iter(|| part2(black_box(&numbers), 25918798))
    });
    group.bench_function("impl B", |b| {
        b.iter(|| part2_b(black_box(&numbers), 25918798))
    });
    group.finish();
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
