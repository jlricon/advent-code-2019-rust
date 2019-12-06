use itertools::iproduct;
enum OpCode {
    Add,
    Mult,
    Halt,
}
impl From<usize> for OpCode {
    fn from(item: usize) -> OpCode {
        use OpCode::*;

        match item {
            1 => Add,
            2 => Mult,
            99 => Halt,
            _ => unreachable!(),
        }
    }
}

fn main() {
    for (i, j) in iproduct!(0..100, 0..100) {
        let res = calc_for_param(i, j);
        if res == 19690720 {
            dbg!(100 * i + j);
            break;
        }
    }
}

fn calc_for_param(i: usize, j: usize) -> usize {
    let mut inp: Vec<usize> = include_str!("day_02_data.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    inp[1] = i;
    inp[2] = j;
    let mut sp: usize = 0;
    loop {
        let op: OpCode = inp[sp].into();
        let p1 = inp[sp + 1];
        let p2 = inp[sp + 2];
        let p3 = inp[sp + 3];
        match op {
            OpCode::Add => {
                inp[p3] = inp[p1] + inp[p2];
            }
            OpCode::Mult => {
                inp[p3] = inp[p1] * inp[p2];
            }
            OpCode::Halt => return inp[0],
        }
        sp += 4
    }
}
