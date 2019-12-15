use advent_code_2019_rust::intcode::Intcode;
use std::collections::HashMap;
use std::fmt::{Error, Formatter};

//0 is an empty tile. No game object appears in this tile.
//1 is a wall tile. Walls are indestructible barriers.
//2 is a block tile. Blocks can be broken by the ball.
//3 is a horizontal paddle tile. The paddle is indestructible.
//4 is a ball tile. The ball moves diagonally and bounces off objects.
#[derive(Eq, PartialEq)]
enum TileKind {
    Empty,
    Wall,
    Block,
    Horizontal,
    Ball,
}
impl std::fmt::Display for TileKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use TileKind::*;
        let val = match self {
            Empty => '.',
            Wall => 'W',
            Block => '#',
            Horizontal => 'P',
            Ball => 'O',
        };
        write!(f, "{}", val)
    }
}

struct Tile {
    pos: [usize; 2],
    kind: TileKind,
}
impl From<&[usize]> for Tile {
    fn from(val: &[usize]) -> Self {
        use TileKind::*;
        let pos = [val[0], val[1]];
        let kind = match val[2] {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Horizontal,
            4 => Ball,
            _ => panic!(),
        };
        Tile { pos, kind }
    }
}
type Field = HashMap<[usize; 2], TileKind>;
fn print_hashmap(p2: Field) {
    let max_x = p2.keys().map(|c| c[0]).max().unwrap();
    let max_y = p2.keys().map(|c| c[1]).max().unwrap();
    let min_x = p2.keys().map(|c| c[0]).min().unwrap();
    let min_y = p2.keys().map(|c| c[1]).min().unwrap();
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            if let Some(kind) = p2.get(&[j, i]) {
                print!("{}", kind);
            } else {
                panic!();
            }
        }
        print!("{}", '\n');
    }
}
fn main() {
    let mut input = Intcode::read_input(include_str!("day_13_data.txt").trim());
    input[0] = 2;
    let mut t = Intcode::new(&input);

    t.compute();
    t.stdin(0);
    t.compute();
    dbg!(t.state);
    let mut v = Vec::new();
    while let Some(out) = t.stdout() {
        v.push(out as usize);
    }
    let tiles: Vec<Tile> = v.chunks_exact(3).map(|v| Tile::from(v)).collect();
    //    let nblock = tiles.iter().filter(|v| v.kind == TileKind::Block).count();
    //    dbg!(nblock);
    let hash: Field = tiles.into_iter().map(|t| (t.pos, t.kind)).collect();
    print_hashmap(hash);
}
