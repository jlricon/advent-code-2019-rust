use advent_code_2019_rust::intcode::Intcode;
use itertools::iproduct;
use std::collections::HashMap;
//fn get_row_len(area: &HashMap<(i64, i64), i64>, row: usize) -> usize {
//    area.iter()
//        .filter(|(k, v)| (k.1 == row as i64) & (**v == 1))
//        .count()
//}
fn part1() {
    let input = Intcode::read_input(include_str!("day_19_data.txt").trim());
    let mut t = Intcode::new(&input);
    let s: usize = 20;
    let area: HashMap<(i64, i64), i64> = iproduct!((0..s as i64), (0..s as i64))
        .map(|(i, j)| {
            t.stdin(i);
            t.stdin(j);
            t.compute();
            let ret = ((i, j), t.stdout().unwrap());
            t.reset();
            ret
        })
        .collect();
    //    // How long are rows 48 and 49
    //
    //    let col_vals: Vec<usize> = (0..100).map(|v| get_row_len(&area, v)).collect();
    //
    //    dbg!(col_vals);
    print_ray(&area, s);
    panic!();
}

fn print_ray(area: &HashMap<(i64, i64), i64>, s: usize) {
    for j in 0..s {
        //        print!("{}", i % 10);
        for i in 0..s {
            if i == 0 {
                print!("{}", j % 10);
            }
            print!("{}", area.get(&(i as i64, j as i64)).unwrap());
        }
        print!("{}", '\n');
    }
}

fn get_from_pos(i: usize, j: usize, t: &mut Intcode) -> usize {
    t.stdin(i as i64);
    t.stdin(j as i64);
    t.compute();
    let ret = t.stdout().unwrap();
    t.reset();
    ret as usize
}
/// Return how many dots are there in that line
/// and also the x position where we last saw a dot
fn scan_line(start_x_at: usize, y: usize, t: &mut Intcode) -> (usize, usize) {
    let mut acc = 0;
    let mut val: usize = 0;
    let mut x = start_x_at;
    let mut proposed_start_x_at = start_x_at;
    let mut are_inside = false;
    let mut empty_sweeps = 0;
    loop {
        val = get_from_pos(x, y, t);
        //        println!("{:?}", (x, y, val));
        if val == 1 {
            acc += val;
            if are_inside == false {
                proposed_start_x_at = x;
            }

            are_inside = true;
        } else if (val == 0) & are_inside {
            break (acc, proposed_start_x_at);
        } else if (empty_sweeps == 6) & !are_inside {
            break (acc, proposed_start_x_at);
        } else {
            empty_sweeps += 1
        }
        x += 1;
    }
}

fn get_down_amount(x: usize, y: usize, t: &mut Intcode) -> usize {
    let mut acc = 0;
    let mut this_y = y;
    let mut val: usize = 0;
    loop {
        val = get_from_pos(x, this_y, t);
        if val == 1 {
            acc += 1;
            this_y += 1;
        } else {
            break acc;
        }
    }
}
fn point_fits(x: usize, y: usize, t: &mut Intcode, s: usize) -> bool {
    // Distance from point to edge
    let x_dist = scan_line(x, y, t).0;
    if x_dist < s {
        //        println!("Failed xdist:{}", x_dist);
        return false;
    }
    let y_dist = get_down_amount(x, y, t);
    if y_dist < s {
        //        println!("Failed ydist:{}", y_dist);
        return false;
    }
    true
}
fn main() {
    //    part1();
    //20->205
    //10->99
    //30->311
    //100~1000?
    let size = 100;
    let input = Intcode::read_input(include_str!("day_19_data.txt").trim());
    let mut t = Intcode::new(&input);
    let mut width_and_x = (0, 0);
    let mut y = 0;
    let xy = 'main: loop {
        println!("Scanning line {}", y);
        width_and_x = scan_line(width_and_x.1, y, &mut t);
        //        dbg!(width_and_x);
        // We can fit if we have >=50
        if width_and_x.0 >= size / 2 {
            // Check them all to be cheap
            for x in width_and_x.0 + width_and_x.1 / 2..width_and_x.1 + width_and_x.0 {
                let it_fits = point_fits(x, y, &mut t, size);
                if it_fits {
                    break 'main (x, y);
                }
            }
        }
        y += 1;
    };
    dbg!(xy.0 * 10000 + xy.1);
}

mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = Intcode::read_input(include_str!("day_19_data.txt").trim());
        let mut t = Intcode::new(&input);
        let v = scan_line(0, 0, &mut t);
        assert_eq!(v, (1, 0));
        let v = scan_line(0, 1, &mut t);
        assert_eq!(v, (0, 0));
        let v = scan_line(3, 5, &mut t);
        assert_eq!(v, (1, 4));
    }
    #[test]
    fn test_four() {
        let input = Intcode::read_input(include_str!("day_19_data.txt").trim());
        let mut t = Intcode::new(&input);
        let v = get_from_pos(4, 5, &mut t);

        assert_eq!(v, 1);
    }
    #[test]
    fn test_2() {
        let input = Intcode::read_input(include_str!("day_19_data.txt").trim());
        let mut t = Intcode::new(&input);
        let v = get_down_amount(15, 17, &mut t);
        assert_eq!(v, 3);
    }
}
