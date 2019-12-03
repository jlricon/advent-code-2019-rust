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
    let mut inp: Vec<usize> = include_str!("day_02_data.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    inp[1] = 12;
    inp[2] = 2;
    let mut sp: usize = 0;
    loop {
        let op: OpCode = inp[sp].into();

        match op {
            OpCode::Add => {
                let p1 = inp[sp + 1];
                let p2 = inp[sp + 2];
                let p3 = inp[sp + 3];
                inp[p3] = inp[p1] + inp[p2];
                sp += 4;
            }
            OpCode::Mult => {
                let p1 = inp[sp + 1];
                let p2 = inp[sp + 2];
                let p3 = inp[sp + 3];
                inp[p3] = inp[p1] * inp[p2];
                sp += 4;
            }
            OpCode::Halt => break,
        }
    }
    dbg!(inp[0]);
}
