use advent_code_2019_rust::intcode::Intcode;
use std::collections::HashSet;

enum Color {
    Black,
    White,
}
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn clockwise(x: &Direction) -> Direction {
        use Direction::*;
        match x {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn counter_clockwise(x: &Direction) -> Direction {
        use Direction::*;
        match x {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
}
type Position = (i32, i32);
struct Robot {
    computer: Intcode,
    pos: Position,
    direction: Direction,
}
impl Robot {
    pub fn walk(&mut self, dir: Direction) -> Position {
        use Direction::*;
        self.direction = dir;
        let dir = match self.direction {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        };
        self.pos = (self.pos.0 + dir.0, self.pos.1 + dir.1);
        self.pos
    }
    pub fn step(&mut self, color: Color) -> Option<(Color, Direction)> {
        use Color::*;
        self.computer.stdin(match color {
            Black => 0,
            White => 1,
        });

        self.computer.compute();
        if self.computer.is_finished() {
            return None;
        }
        let paint_action = self.computer.stdout().unwrap();
        let turn_action = self.computer.stdout().unwrap();
        Some((
            match paint_action {
                0 => Black,
                1 => White,
                _ => panic!(),
            },
            match turn_action {
                0 => Direction::counter_clockwise(&self.direction),
                1 => Direction::clockwise(&self.direction),
                _ => panic!(),
            },
        ))
    }
}
fn main() {
    let input = Intcode::read_input(include_str!("day_11_data.txt").trim());

    let t = Intcode::new(&input);
    let mut robot = Robot {
        computer: t,
        pos: (0, 0),
        direction: Direction::Up,
    };
    let mut points = HashSet::new();
    while let Some(outcome) = robot.step(Color::White) {
        let newpos = robot.walk(outcome.1);
        points.insert(newpos);
    }
    dbg!(points.len());
}
