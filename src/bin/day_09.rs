use advent_code_2019_rust::intcode::{Intcode, IntcodeProg};

pub struct Day09 {
    input: IntcodeProg,
}

// Shamelessly stolen from https://github.com/AxlLind/AdventOfCode2019/blob/master/src/bin/9.rs

fn run1() -> i64 {
    let input = Intcode::read_input(include_str!("day_09_data.txt").trim());

    let mut t = Intcode::new(&input);

    t.stdin(1);

    t.compute();

    return t.stdout().unwrap();
}

fn run2() -> i64 {
    let input = Intcode::read_input(include_str!("day_09_data.txt").trim());

    let mut t = Intcode::new(&input);

    t.stdin(2);

    t.compute();

    return t.stdout().unwrap();
}

fn main() {
    dbg!(run1());
    dbg!(run2());
}
