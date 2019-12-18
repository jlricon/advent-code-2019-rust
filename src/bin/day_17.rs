use advent_code_2019_rust::intcode::Intcode;
use itertools::iproduct;
fn print_vec(v: &Vec<i64>) {
    v.iter().for_each(|n| print!("{}", *n as u8 as char))
}
const NEWLINE: i64 = 10;
const BLOCK: i64 = 35;
const DIRS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
fn main() {
    let input = Intcode::read_input(include_str!("day_17_data.txt").trim());
    let out = input_to_vec(&input);
    let inters = part1(&out);

    dbg!(inters);
}

fn input_to_vec(input: &Vec<i64>) -> Vec<i64> {
    let mut p = Intcode::new(&input);
    p.compute();
    let mut out = Vec::new();
    while let Some(v) = p.stdout() {
        out.push(v)
    }
    out
}
fn get_xy(row: i64, col: i64, v: &Vec<i64>, x_len: i64) -> Option<&i64> {
    if col >= x_len as i64 {
        return None;
    }
    (&v).get((col + (x_len) * row) as usize)
}
fn is_intersec(row: usize, col: usize, out: &Vec<i64>, x_len: i64) -> bool {
    DIRS.iter()
        .filter_map(|v| get_xy(v.0 + row as i64, v.1 + col as i64, out, x_len))
        .filter(|&&v| v == BLOCK)
        .count()
        == 4
}
fn part1(out_inp: &Vec<i64>) -> usize {
    let x_len = out_inp.iter().take_while(|p| **p != NEWLINE).count() as i64;
    let out: Vec<i64> = out_inp
        .into_iter()
        .filter(|p| **p != NEWLINE)
        .map(|v| *v)
        .collect();
    let y_len = out.len() as i64 / x_len;
    dbg!(x_len);

    let inters: usize = iproduct!(0..x_len as usize, 0..y_len as usize)
        .map(|(row, col)| ((row, col), is_intersec(row, col, &out, x_len)))
        .filter_map(|((r, c), is_in)| match is_in {
            true => Some(c * r),
            false => None,
        })
        .sum();
    inters
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_1() {
        let input = "..#............#..........##O####...####.#...#...#.###O###O###O##..#...#...#....#####...^.."
            .chars()
            .map(|v| v as i64)
            .collect_vec();
        assert_eq!(46, *get_xy(0, 0, &input, 13).unwrap());
        // (col + x_len * row) (2+13*0)-2
        assert_eq!(BLOCK, *get_xy(0, 2, &input, 13).unwrap());
        // (col + x_len * row) (2+13)-15 (real pos 16)
        assert_eq!(BLOCK, *get_xy(1, 2, &input, 13).unwrap());
        assert_eq!(BLOCK, *get_xy(6, 4, &input, 13).unwrap());
        assert_eq!(false, is_intersec(0, 0, &input, 13));
        assert_eq!(true, is_intersec(2, 2, &input, 13));
        assert_eq!(true, is_intersec(4, 6, &input, 13));
        assert_eq!(false, is_intersec(2, 12, &input, 13));
    }
}
