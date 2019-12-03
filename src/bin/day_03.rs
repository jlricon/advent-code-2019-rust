use std::ops::Add;

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

#[derive(Debug, Clone, Copy)]
struct Point(u32, u32);
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
    let mut start = vec![Point(0, 0)];
    steps.iter().for_each(|x| match x {
        Step {
            dir: Directions::D,
            num,
        } => start.push(start.last().unwrap().displace(Point(0, 1))),
        _ => unreachable!(),
    });
    start
}
fn main() {
    let res: Vec<Vec<Point>> = include_str!("day_03_data.txt")
        .lines()
        .map(|x| x.split(',').map(|x| Step::from(x)).collect())
        .map(get_vector)
        .collect();
    dbg!(res);
}
