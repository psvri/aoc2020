use aoc2020::{problems::day1::Day1, solve_seq};
use paste::paste;

use aoc2020::problems::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_problems {
    { $c: ident, $($days:literal),* } => {
        $(
            paste! {
                $c.bench_function("day 1", |b| {
                    b.iter(|| [<Day $days>]::solve(black_box([<Day $days>]::PROBLEM_INPUT)))
                });
            }
        )*
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_problems! {c, 1}
    c.bench_function("day seq", |b| b.iter(|| solve_seq()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
