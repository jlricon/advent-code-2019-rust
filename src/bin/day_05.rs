use advent_code_2019_rust::day5::run_computer;

use std::iter::FromIterator;

fn main() {
    let inp: Vec<i64> = include_str!("day_05_data.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let input_val = Vec::from_iter(std::iter::once(5));
    run_computer(&inp, input_val, 0);
}
