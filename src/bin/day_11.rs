use advent_code_2019_rust::intcode::Intcode;
use std::collections::HashMap;
use std::fmt::{Error, Formatter};

#[derive(Debug, Copy, Clone)]
enum Color {
    Black,
    White,
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let val = match self {
            Color::Black => '.',
            Color::White => '#',
        };
        write!(f, "{}", val)
    }
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
    // Part 1
    dbg!(run_for_starting_point(Color::Black).len());
    // Part 2
    let p2 = run_for_starting_point(Color::White);
    // Max width
    let max_x = p2.keys().map(|c| c.0).max().unwrap();
    let max_y = p2.keys().map(|c| c.1).max().unwrap();
    let min_x = p2.keys().map(|c| c.0).min().unwrap();
    let min_y = p2.keys().map(|c| c.1).min().unwrap();
    for i in (min_y..=max_y).rev() {
        for j in min_x..=max_x {
            if let Some(color) = p2.get(&(j, i)) {
                print!("{}", color);
            } else {
                print!("{}", Color::Black)
            }
        }
        print!("{}", '\n');
    }
}

fn run_for_starting_point(start_color: Color) -> HashMap<(i32, i32), Color> {
    let input = Intcode::read_input(include_str!("day_11_data.txt").trim());
    let t = Intcode::new(&input);
    let mut robot = Robot {
        computer: t,
        pos: (0, 0),
        direction: Direction::Up,
    };
    let mut points = HashMap::new();
    let mut colorbelow = start_color;
    let mut pos = (0, 0);
    while let Some((color, direction)) = robot.step(colorbelow) {
        points.insert(pos, color);

        pos = robot.walk(direction);
        if let Some(color) = points.get(&pos) {
            colorbelow = *color;
        } else {
            colorbelow = Color::Black;
        }
    }
    points
}
