use advent_code_2019_rust::day6::{get_jumps, str_to_orbits};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::path;

fn benchf(tst: &str) {
    let orbits = str_to_orbits(tst);
    let jumps = get_jumps(orbits);
}
fn criterion_benchmark(c: &mut Criterion) {
    let path = path::Path::new("./src/bin/day_06_data.txt").canonicalize();
    let val = fs::read_to_string(path.unwrap()).unwrap();

    c.bench_function("day_6", |b| b.iter(|| benchf(black_box(&val))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
