use itertools::iproduct;
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

const PI: f32 = std::f32::consts::PI;
#[derive(Debug, PartialEq, Clone)]
enum Matter {
    Asteroid,
    Empty,
    Out,
}
#[derive(Debug)]
struct Field {
    stuff: Vec<Matter>,
    ncol: usize,
    nrow: usize,
}
type Vector = (i32, i32);
type Position = (usize, usize);
type Angle = f32;
fn gcd(i: Vector) -> usize {
    let mut x = i.0.abs();
    let mut y = i.1.abs();
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x as usize
}

fn norm(x: Vector) -> Vector {
    let gcd = gcd(x);
    if gcd == 0 {
        x
    } else {
        (x.0 / gcd as i32, x.1 / gcd as i32)
    }
}
fn vec_angle(v: Vector) -> Angle {
    let x = (v.0 as f32, v.1 as f32);
    //    let up = (0, -1);
    //    let r = f32::acos(x.1 / f32::sqrt(x.0 * x.0 + x.1 * x.1));

    let at = (v.0 as f32).atan2(v.1 as f32);
    if v.0 < 0 {
        at + 2.0 * std::f32::consts::PI
    } else {
        at
    }
}
fn euclid(v: Vector) -> usize {
    (v.0 * v.0 + v.1 * v.1) as usize
}
fn sort_by_angle_and_magnitude(a: Position, c: Position, refpos: Position) -> Ordering {
    let v = (a.0 as i32 - refpos.0 as i32, a.1 as i32 - refpos.1 as i32);
    let b = (c.0 as i32 - refpos.0 as i32, c.1 as i32 - refpos.1 as i32);
    let angle1 = vec_angle(v);
    let angle2 = vec_angle(b);

    if (angle1 - angle2).abs() < 0.001 {
        let norm1 = euclid(v);
        let norm2 = euclid(b);
        dbg!(v, b, angle1, angle2, refpos);
        if norm1 < norm2 {
            Ordering::Less
        } else if norm1 > norm2 {
            Ordering::Greater
        } else {
            panic!("Two asteroids in the same place!");
        }
    } else if angle1 > angle2 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

impl Field {
    pub fn get_asteroids_visible_for_all(&self) -> Vec<(Position, Option<usize>)> {
        let prod: Vec<(usize, usize)> = iproduct!(1..=self.nrow, 1..=self.ncol).collect();
        use Matter::*;
        prod.into_iter()
            .map(|rcol| match self.get(rcol.0 as i32, rcol.1 as i32) {
                Empty => (rcol, None),
                Asteroid => (rcol, Some(self.get_n_visible_asteroids(rcol.0, rcol.1))),
                Out => panic!(),
            })
            .collect()
    }
    fn get_n_visible_asteroids(&self, row: usize, col: usize) -> usize {
        self.get_asteroids_by_angle(row, col)
            .iter()
            .unique()
            .count()
    }
    pub fn get_asteroids_by_angle(&self, row: usize, col: usize) -> Vec<Vector> {
        // Given a position, return a list of the vectors pointing to everything around it
        use Matter::*;
        iproduct!(1..=self.nrow, 1..=self.ncol)
            .filter(|c| if c == &(row, col) { false } else { true })
            .filter_map(
                |(thisrow, thiscol)| match self.get(thisrow as i32, thiscol as i32) {
                    Empty => None,
                    Out => panic!(),
                    Asteroid => {
                        let vec = (thisrow as i32 - row as i32, thiscol as i32 - col as i32);

                        Some(norm(vec))
                    }
                },
            )
            .collect()
    }
    pub fn find_vaporize_n(&self, pos: Position, target: usize) -> Position {
        let asteroid_angles = self.get_asteroids_by_angle(pos.0, pos.1);
        //        dbg!(&asteroid_angles);
        let sorted_positions = self.get_sorted_positions(asteroid_angles);
        //        dbg!(&sorted_positions);
        // Hashmap with vector->points, sorted by distance from original point
        let mut counted_vectors = self.get_counted_vectors(pos, &sorted_positions);
        dbg!(&counted_vectors);
        let mut zapped = 0;
        let mut sorted_vectors = sorted_positions.into_iter().map(|v| v.0).cycle();
        loop {
            let elem = sorted_vectors.next().unwrap();
            let pos = counted_vectors.get_mut(&elem).unwrap().pop();
            match pos {
                None => continue,
                Some(pos) => {
                    dbg!(pos, (pos.0 - 1) * 100 + pos.1 - 1);
                    zapped += 1;
                }
            }

            if zapped == target {
                break pos.unwrap();
            }
        }

        // We can get the 200th if we have an ordering (by clockwiseness), then iterate
    }

    fn get_counted_vectors(
        &self,
        pos: Position,
        sorted_positions: &Vec<(Vector, Angle)>,
    ) -> HashMap<Vector, Vec<Position>> {
        // Given a position, a a sorted ordering of (vector,angle)
        sorted_positions
            .iter()
            .map(|k| k.0)
            .group_by(|k| *k)
            .into_iter()
            // Here we need to collect & sort the vecs by magnitude, with smaller at the end
            .map(|(k, v)| {
                (
                    k,
                    v.map(|r| ((r.0 + pos.0 as i32) as usize, (r.1 + pos.1 as i32) as usize))
                        .sorted_by_key(|x| ((x.0 * x.0 + x.1 * x.1) as i32))
                        .collect(),
                )
            })
            .collect()
    }

    fn get_sorted_positions(&self, asteroid_angles: Vec<(i32, i32)>) -> Vec<(Vector, Angle)> {
        asteroid_angles
            .iter()
            .map(|f| (*f, vec_angle(*f)))
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Equal))
            .rev()
            .collect()
    }
    pub fn get(&self, row: i32, col: i32) -> &Matter {
        if (col > self.ncol as i32) | (row > self.nrow as i32) | (row < 1) | (col < 1) {
            &Matter::Out
        } else {
            &self.stuff[((row - 1) * self.ncol as i32 + (col - 1)) as usize]
        }
    }
    pub fn print(&self) {
        self.stuff.chunks_exact(self.nrow).for_each(|v| {
            println!("{:?}", v);
        })
    }
    pub fn from_vec(v: Vec<&str>) -> Field {
        use Matter::{Asteroid, Empty};
        let nrow = v.len();
        let ncol = v.iter().map(|v| v.len()).collect_vec();
        assert!(ncol.iter().all_equal());
        let ncol = ncol[0];
        let stuff = v
            .into_iter()
            .map(|c| c.chars())
            .flatten()
            .map(|c| match c {
                '.' => Empty,
                '#' => Asteroid,
                _ => panic!("Unexpected input"),
            })
            .collect();
        Field { stuff, ncol, nrow }
    }
}
fn get_input() -> Vec<&'static str> {
    include_str!("day_10_data.txt").lines().collect_vec()
}
fn main() {
    let input = get_input();
    let field = Field::from_vec(input);
    let visibles = field.get_asteroids_visible_for_all();
    let max = visibles
        .iter()
        .filter_map(|v| match v {
            (_, None) => None,
            (rowcol, Some(val)) => Some((rowcol, val)),
        })
        .max_by_key(|k| k.1)
        .unwrap();
    dbg!(max);
    field.find_vaporize_n(*max.0, 200);
}

#[cfg(test)]
mod tests {
    use super::Matter::{Asteroid, Empty, Out};
    use super::*;

    use std::cmp::Ordering::Equal;

    use std::iter::FromIterator;

    #[test]
    fn test_vap() {
        let data: Vec<&str> = "#.#\n.#.\n#.#".split('\n').collect();
        let field = Field::from_vec(data);
        let ret = field.find_vaporize_n((2, 2), 1);
        assert_eq!(ret, (1, 3));
    }
    #[test]
    fn test_field() {
        let field = Field {
            stuff: vec![Asteroid, Empty, Empty, Asteroid],
            ncol: 2,
            nrow: 2,
        };
        assert_eq!(field.get(1, 1), &Asteroid);
        assert_eq!(field.get(1, 2), &Empty);
        assert_eq!(field.get(2, 2), &Asteroid);
        assert_eq!(field.get(2, 3), &Out);
        let field = Field {
            stuff: vec![Asteroid, Asteroid, Empty, Empty, Asteroid, Empty],
            ncol: 3,
            nrow: 2,
        };
        assert_eq!(field.get(1, 1), &Asteroid);
        assert_eq!(field.get(1, 2), &Asteroid);
        assert_eq!(field.get(1, 3), &Empty);
        assert_eq!(field.get(2, 1), &Empty);
        assert_eq!(field.get(2, 2), &Asteroid);
        assert_eq!(field.get(2, 3), &Empty);
    }
    #[test]
    fn test_detect() {
        let data: Vec<&str> = ".#..#\n.....\n#####\n....#\n...##".split('\n').collect();
        let field = Field::from_vec(data);
        let asteroids = field.get_asteroids_by_angle(3, 1);
        assert_eq!(asteroids.iter().unique().count(), 6);
        let asteroids = field.get_asteroids_by_angle(1, 5);
        assert_eq!(asteroids.len(), 9);
        let asteroids = field.get_asteroids_by_angle(5, 4);
        assert_eq!(asteroids.iter().unique().count(), 8);
        let visibles = field.get_asteroids_visible_for_all();
        let max = visibles.iter().filter_map(|v| v.1).max().unwrap();
        assert_eq!(8, max)
    }
    #[test]
    fn test_detect2() {
        let data: Vec<&str> = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            .split('\n')
            .collect();
        let field = Field::from_vec(data);
        let visibles = field.get_asteroids_visible_for_all();
        let max = visibles.iter().filter_map(|v| v.1).max().unwrap();
        assert_eq!(33, max)
    }
    #[test]
    fn test_ordering() {
        let vecs = vec![(0, 1), (1, 1), (1, 0), (0, -1), (-1, 0), (-2, 0)];
        let ord: Vec<f32> = vecs.iter().map(|f| vec_angle(*f)).collect();
        let mut ordered: Vec<f32> = ord.clone();
        ordered.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        assert_eq!(ordered, ord);
    }
    #[test]
    fn test_angle() {
        let ans = vec_angle((0, 1));
        assert_eq!(ans, 0.0);
        let ans = vec_angle((1, 0));
        assert_eq!(ans, PI / 2_f32);
        let ans = vec_angle((0, -1));
        assert!(ans - PI < 1e-5);
        let ans = vec_angle((-1, 0));
        assert_eq!(ans, 3_f32 * PI / 2_f32);
        let ans = vec_angle((-1, 1));
        assert_eq!(ans, 1.75_f32 * PI);
    }

    #[test]
    fn test_sorted_pos() {
        let data: Vec<&str> = "#.#\n.#.\n#.#".split('\n').collect();
        let field = Field::from_vec(data);
        let angles = field.get_asteroids_by_angle(2, 2);
        let res = field.get_sorted_positions(angles);

        assert_eq!(
            res.iter().map(|v| v.0).collect_vec(),
            vec![(1, 1), (1, -1), (-1, -1), (-1, 1)]
        );
    }
    #[test]
    fn test_asteroids_angle() {
        let data: Vec<&str> = "#.#\n.#.\n#.#".split('\n').collect();
        let field = Field::from_vec(data);
        let angles = field.get_asteroids_by_angle(1, 1);
        assert_eq!(angles, vec![(0, 1), (1, 1), (1, 0), (1, 1)]);
        let res = field.get_sorted_positions(angles);
        let sorted = res.iter().map(|i| i.0).collect_vec();
        assert_eq!(sorted, vec![(0, 1), (1, 1), (1, 1), (1, 0)]);
    }
    #[test]
    fn test_sort_absolute() {
        let vecs = vec![(1, 2), (2, 2)];
        let pos = (3, 2);
        let sorted = vecs
            .into_iter()
            .sorted_by(|a, b| sort_by_angle_and_magnitude(*a, *b, pos))
            .collect_vec();
        assert_eq!(sorted, vec![(2, 2), (1, 2)]);
        let vecs = vec![(1, 1), (1, 3)];
        let pos = (2, 2);
        let sorted = vecs
            .into_iter()
            .sorted_by(|a, b| sort_by_angle_and_magnitude(*a, *b, pos))
            .collect_vec();
        assert_eq!(sorted, vec![(1, 3), (1, 1)]);

        let vecs = vec![(1, 1), (1, 3), (1, 2)];
        let pos = (2, 2);
        let sorted = vecs
            .into_iter()
            .sorted_by(|a, b| sort_by_angle_and_magnitude(*a, *b, pos))
            .collect_vec();
        assert_eq!(sorted, vec![(1, 2), (1, 3), (1, 1)]);
        dbg!(sorted);
    }
}
