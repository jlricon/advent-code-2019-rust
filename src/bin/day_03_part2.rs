use std::collections::HashSet;
use std::iter::FromIterator;

enum Directions {
    U,
    D,
    R,
    L,
}

struct Step {
    dir: Directions,
    num: u32,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point(i32, i32);
impl Point {
    fn displace(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl From<&str> for Step {
    fn from(item: &str) -> Self {
        use crate::Directions::*;
        let dir = match item.chars().nth(0).unwrap() {
            'D' => D,
            'U' => U,
            'R' => R,
            'L' => L,
            _ => unreachable!(),
        };
        let num = item[1..].parse().unwrap();
        Step { dir, num }
    }
}

fn get_vector(steps: Vec<Step>) -> Vec<Point> {
    fn push_into_vec_n(v: &mut Vec<Point>, num: u32, point: Point) -> () {
        (0..num).for_each(|_| v.push(v.last().unwrap().displace(&point)))
    }
    let mut v = vec![Point(0, 0)];
    use Directions::*;

    steps.iter().for_each(|x| match x {
        Step { dir: D, num } => push_into_vec_n(&mut v, *num, Point(0, -1)),
        Step { dir: U, num } => push_into_vec_n(&mut v, *num, Point(0, 1)),
        Step { dir: R, num } => push_into_vec_n(&mut v, *num, Point(1, 0)),
        Step { dir: L, num } => push_into_vec_n(&mut v, *num, Point(-1, 0)),
    });
    v
}

fn solve(input: &str) {
    let points = input
        .lines()
        .map(|x| x.split(',').map(|x| x.into()).collect())
        .map(get_vector)
        .collect::<Vec<Vec<Point>>>();
    let first_line: HashSet<&Point> = HashSet::from_iter(&points[0]);

    // For each intersection, find the number of steps to it
    let min_dist = points[1]
        .iter()
        .filter(|x| first_line.contains(x))
        .map(|intr| {
            points[0].iter().position(|x| x == intr).unwrap()
                + points[1].iter().position(|x| x == intr).unwrap()
        })
        .filter(|x| *x != 0)
        .min();

    dbg!(min_dist);
}
fn main() {
    let points = include_str!("day_03_data.txt");
    solve(points);
}
