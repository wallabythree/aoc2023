use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use rudolf_rs;
use solutions::*;

fn criterion_benchmark(c: &mut Criterion) {
    let session_key = env::var("AOC_SESSION").unwrap();
    let client = rudolf_rs::Client::new(String::from(session_key));

    c.bench_function(
        "day00part1",
        |b| {
            b.iter(|| day00::part1(black_box("0")))
        }
    );

    c.bench_function(
        "day00part2",
        |b| {
            b.iter(|| day00::part1(black_box("")))
        }
    );

    /*
    let mut input = client.get(2023, 1).unwrap();

    c.bench_function(
        "day01part1",
        |b| {
            b.iter(|| day01::part1(black_box(&input)))
        }
    );

    c.bench_function(
        "day01part2",
        |b| {
            b.iter(|| day01::part2(black_box(&input)))
        }
    );
    */
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);

