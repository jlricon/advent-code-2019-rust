use advent_code_2019_rust::intcode::Intcode;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone)]
enum Moves {
    West,
    South,
    East,
    North,
}
#[derive(Debug)]
enum Status {
    HitWall,
    Success,
    Found,
}
#[derive(Debug)]
enum Tile {
    Block,
    Empty,
    Goal,
}
struct PositionedBlock(usize, Tile);
impl Moves {
    fn clockwise(&self) -> Self {
        use Moves::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
impl From<i64> for Status {
    fn from(a: i64) -> Self {
        match a {
            0 => Status::HitWall,
            1 => Status::Success,
            2 => Status::Found,
            _ => panic!(),
        }
    }
}
struct Vector(i32, i32);
impl Into<Vector> for Moves {
    fn into(self) -> Vector {
        use Moves::*;
        match self {
            North => Vector(0, 1),
            East => Vector(1, 0),
            South => Vector(0, -1),
            West => Vector(-1, 0),
        }
    }
}
impl Into<i64> for Moves {
    fn into(self) -> i64 {
        use Moves::*;
        match self {
            North => 1,
            East => 4,
            South => 2,
            West => 3,
        }
    }
}
impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}
fn part1() {
    use Moves::*;
    let input = Intcode::read_input(include_str!("day_15_data.txt").trim());

    let mut t = Intcode::new(&input);
    let mut next_move = North;
    let mut pos = (0, 0);
    let mut universe = HashMap::new();
    let mut potential_pos = Vector(0, 1);
    let mut delta = Vector(0, -1);
    universe.insert(pos, PositionedBlock(0, Tile::Empty));
    'main: loop {}
    dbg!(universe);
}
fn main() {
    part1()
}

#[cfg(test)]
mod test {
    use crate::Moves;
    use crate::Moves::*;
    use itertools::Itertools;

    #[test]
    fn test_move_ord() {
        let ords = vec![North, East, West];
        let mut ordered = ords.clone();
        ordered.sort();
        assert_eq!(ordered, vec![West, East, North])
    }
}
