use advent_code_2019_rust::intcode::Intcode;
use itertools::Itertools;
use pathfinding::directed::dijkstra;
use std::collections::HashMap;
use std::io::Write;
#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Hash)]
enum Moves {
    North,
    East,
    South,
    West,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Status {
    HitWall,
    Success,
    Found,
}
#[derive(Debug, PartialEq, Hash, PartialOrd, Eq)]
enum Tile {
    Block,
    Empty,
    Goal,
}
impl Into<Tile> for Status {
    fn into(self) -> Tile {
        use Status::*;
        use Tile::*;
        match self {
            HitWall => Block,
            Success => Empty,
            Found => Goal,
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
struct PositionedBlock(usize, Tile);

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
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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
fn print_universe(universe: &Universe) {
    let min_x = universe.keys().map(|m| m.0).min().unwrap();
    let min_y = universe.keys().map(|m| m.1).min().unwrap();
    let max_x = universe.keys().map(|m| m.0).max().unwrap();
    let max_y = universe.keys().map(|m| m.1).max().unwrap();
    let mut file = std::fs::File::create("day_15_out.txt").unwrap();
    for i in (min_y..=max_y).rev() {
        for j in min_x..=max_x {
            let block = universe.get(&Vector(j, i));

            match block {
                None => file.write("N".as_bytes()),
                Some(_) if (j, i) == (0, 0) => file.write("O".as_bytes()),
                Some(PositionedBlock(_, Tile::Block)) => file.write("#".as_bytes()),
                Some(PositionedBlock(u, Tile::Empty)) => {
                    file.write(format!("{}", u / 100).as_bytes())
                }
                Some(PositionedBlock(_, Tile::Goal)) => file.write("G".as_bytes()),
            }
            .unwrap();
        }

        file.write("\n".as_bytes()).unwrap();
    }
}
type Universe = HashMap<Vector, PositionedBlock>;
fn part1() -> (usize, Universe) {
    use Moves::*;
    use Status::*;
    let input = Intcode::read_input(include_str!("day_15_data.txt").trim());

    let mut t = Intcode::new(&input);
    let mut pos = Vector(0, 0);
    let mut universe = HashMap::new();
    let mut distance_from_origin = 0;
    universe.insert(pos, PositionedBlock(0, Tile::Empty));
    let mut nit = 0;
    let mut found_goal = false;

    'main: loop {
        // What is around us?
        let blocks_around: HashMap<Moves, Option<&PositionedBlock>> = [North, East, South, West]
            .iter()
            .map(|m| (*m, Into::<Vector>::into(*m) + *&pos))
            .map(|(m, p)| (m, universe.get(&p)))
            .collect();

        // Is there unexplored territory?
        let unexplored = blocks_around
            .iter()
            .filter_map(|(k, v)| match v {
                None => Some(k),
                Some(_) => None,
            })
            //Exploration order depends on found_goal
            //            .sorted_by(|a, b| {
            //                let comparison = a.cmp(b);
            //                if found_goal {
            //                    comparison.reverse()
            //                } else {
            //                    comparison
            //                }
            //            })
            .nth(0);
        // ALways take the shortest path to the beginning
        let visited = blocks_around
            .iter()
            .filter_map(|(k, v)| match v {
                None => None,
                Some(PositionedBlock(_, Tile::Empty)) => Some((k, v.unwrap())),
                _ => None,
            })
            .sorted_by_key(|k| (k.1).0)
            .nth(0);
        let next_move = match unexplored {
            Some(m) => m,
            None => visited.unwrap().0,
        };
        t.stdin((*next_move).into());
        t.compute();
        let status: Status = t.stdout().unwrap().into();

        let new_pos: Vector = pos + (*next_move).into();
        match status {
            Found | Success => {
                pos = new_pos;
            }
            HitWall => (),
        }
        // If we have already seen the point, compute effective distance from origin
        match unexplored {
            // We moved to an unexplored block
            Some(_) => distance_from_origin += 1,
            // We moved to a previously visited block
            None => {
                let seen_block_distance = (visited.unwrap().1).0;
                distance_from_origin = seen_block_distance.min(distance_from_origin + 1);
            }
        }

        universe.insert(
            new_pos,
            PositionedBlock(distance_from_origin, status.into()),
        );
        // If we found the final point
        if (status == Found) | (found_goal & (pos == Vector(0, 0))) {
            if found_goal {
                break (distance_from_origin, universe);
            } else {
                found_goal = true;
                // Reset intcode machine
                //                t = Intcode::new(&input);
                //                pos = Vector(0, 0);
                //                distance_from_origin = 0;
            }
        }

        nit += 1;
        if nit > 20000 {
            panic!();
        }
    }
}

fn main() {
    use Moves::*;
    let universe = part1().1;
    let goal = universe
        .iter()
        .filter(|p| (p.1).1 == Tile::Goal)
        .nth(0)
        .unwrap()
        .0;
    let all_empty_blocks: Vec<&Vector> = universe
        .iter()
        .filter(|(vector, block)| block.1 == Tile::Empty)
        .map(|v| v.0)
        .collect();
    // Apply A* algo
    let succ = |v: &Vector| {
        let blocks_around: Vec<(Vector, usize)> = [North, East, South, West]
            .iter()
            .map(|m| Into::<Vector>::into(*m) + *v)
            .map(|p: Vector| (p, universe.get(&p)))
            .filter_map(|(p, block)| match block {
                None => None,
                Some(v) if v.1 != Tile::Block => Some((p, 1)),
                _ => None,
            })
            .collect();
        blocks_around
    };
    let shortest = dijkstra::dijkstra(&Vector(0, 0), succ, |p| p == goal);
    dbg!(shortest.unwrap().1);
    // For part 2, do Dijkstra on everything lol. This doesn't seem efficient, but.
    let longest = all_empty_blocks
        .iter()
        .map(|v| dijkstra::dijkstra(goal, succ, |p| p == *v).unwrap().1)
        .max()
        .unwrap();
    dbg!(longest);
}

#[cfg(test)]
mod test {
    use crate::Moves::*;
    use itertools::Itertools;

    #[test]
    fn test_move_ord() {
        let ords = vec![North, East, West];
        let mut ordered = ords.clone();
        ordered.sort();
        assert_eq!(ordered, vec![North, East, West])
    }
}
