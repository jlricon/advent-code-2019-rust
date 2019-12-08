use advent_code_2019_rust::day5::run_five;
use itertools::Itertools;

fn main() {
    let inp: Vec<i64> = include_str!("day_07_data.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let phase = vec![6, 5, 7, 8, 9];
    let p: Vec<Vec<i64>> = phase.into_iter().permutations(5).collect();
    let m = p
        .into_iter()
        .map(|phase| {
            let r = run_five(&inp, phase);
            dbg!("Finished a set of perms");
            r
        })
        .max();
    dbg!(m);
}
