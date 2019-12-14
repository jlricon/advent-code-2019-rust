use advent_code_2019_rust::intcode::Intcode;

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
fn main() {
    let input = Intcode::read_input(include_str!("day_13_data.txt").trim());

    let mut t = Intcode::new(&input);
    while !t.is_finished() {
        t.compute();
    }
    let mut v = Vec::new();
    while let Some(out) = t.stdout() {
        v.push(out as usize);
    }
    let tiles: Vec<Tile> = v.chunks_exact(3).map(|v| Tile::from(v)).collect();
    let nblock = tiles.iter().filter(|v| v.kind == TileKind::Block).count();
    dbg!(nblock);
}
