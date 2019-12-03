use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
enum Directions {
    U,
    D,
    R,
    L,
}
#[derive(Debug)]
struct Step {
    dir: Directions,
    num: u32,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point(i32, i32);
impl Point {
    fn displace(&self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl From<&str> for Step {
    fn from(item: &str) -> Self {
        let dir = match item.chars().nth(0).unwrap() {
            'D' => Directions::D,
            'U' => Directions::U,
            'R' => Directions::R,
            'L' => Directions::L,
            _ => unreachable!(),
        };
        let num = item[1..].parse().unwrap();
        Step { dir, num }
    }
}

fn get_vector(steps: Vec<Step>) -> Vec<Point> {
    let mut v = vec![Point(0, 0)];
    steps.iter().for_each(|x| match x {
        Step {
            dir: Directions::D,
            num,
        } => (0..*num).for_each(|_| push_into_vec(&mut v, Point(0, -1))),
        Step {
            dir: Directions::U,
            num,
        } => (0..*num).for_each(|_| push_into_vec(&mut v, Point(0, 1))),
        Step {
            dir: Directions::R,
            num,
        } => (0..*num).for_each(|_| push_into_vec(&mut v, Point(1, 0))),
        Step {
            dir: Directions::L,
            num,
        } => (0..*num).for_each(|_| push_into_vec(&mut v, Point(-1, 0))),
    });
    v
}
fn manhattan(a: &Point) -> i32 {
    a.0.abs() + a.1.abs()
}
fn push_into_vec(v: &mut Vec<Point>, point: Point) {
    v.push(v.last().unwrap().displace(point));
}

fn main() {
    let points = include_str!("day_03_data.txt")
        .lines()
        .map(|x| x.split(',').map(|x| Step::from(x)).collect())
        .map(get_vector)
        .map(HashSet::from_iter)
        .collect::<Vec<HashSet<Point>>>();

    points[0]
        .intersection(&(points[1]))
        .map(manhattan)
        .filter(|x| *x != 0)
        .min()
        .map(|x| dbg!(x));
}
