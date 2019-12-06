use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl From<char> for Mode {
    fn from(item: char) -> Mode {
        use Mode::*;

        match item {
            '0' => Position,
            '1' => Immediate,
            _ => panic!("Invalid Mode"),
        }
    }
}
#[derive(Debug, PartialEq)]
enum Instruction {
    Halt,
    Save,
    Add { modes: [Mode; 3] },
    Output { modes: Mode },
    Mult { modes: [Mode; 3] },
    JumpIfTrue { modes: [Mode; 2] },
    JumpIfFalse { modes: [Mode; 2] },
    LessThan { modes: [Mode; 3] },
    Equals { modes: [Mode; 3] },
}
const INPUT: i32 = 5;
impl Instruction {
    fn get_step(&self) -> usize {
        use Instruction::*;
        match self {
            Halt => unreachable!(),
            Save | Output { .. } => 2,
            Add { .. } | Mult { .. } | LessThan { .. } | Equals { .. } => 4,
            JumpIfFalse { .. } | JumpIfTrue { .. } => 3,
        }
    }

    fn apply(&self, vec: &mut Vec<i32>, pc: usize) -> usize {
        use Instruction::*;
        use Mode::*;
        fn get_vals(vec: &Vec<i32>, modes: Vec<Mode>, pc: usize) -> (i32, i32) {
            use Mode::*;
            let v1 = match modes[0] {
                Position => vec[vec[pc + 1] as usize],
                Immediate => vec[pc + 1],
            };
            let v2 = match modes[1] {
                Position => vec[vec[pc + 2] as usize],
                Immediate => vec[pc + 2],
            };

            (v1, v2)
        }

        match self {
            JumpIfTrue { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                if v1 != 0 {
                    v2 as usize
                } else {
                    pc + self.get_step()
                }
            }
            JumpIfFalse { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                if v1 == 0 {
                    v2 as usize
                } else {
                    pc + self.get_step()
                }
            }
            Equals { modes } => {
                let pos = match modes[2] {
                    Position => vec[pc + 3] as usize,
                    Immediate => panic!("No"),
                };
                let (v1, v2) = get_vals(&vec, modes.to_vec(), pc);

                if v1 == v2 {
                    vec[pos] = 1;
                } else {
                    vec[pos] = 0;
                }
                pc + self.get_step()
            }
            LessThan { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let pos = match modes[2] {
                    Position => vec[pc + 3] as usize,
                    Immediate => panic!(),
                } as usize;
                if v1 < v2 {
                    vec[pos] = 1
                } else {
                    vec[pos] = 0
                };
                pc + self.get_step()
            }
            Halt => panic!(),
            Output { modes } => {
                let pos1 = vec[pc + 1] as usize;
                match modes {
                    Position => println!("{}", vec[pos1 as usize]),
                    Immediate => println!("{}", pos1),
                }
                pc + self.get_step()
            }
            Save => {
                let pos = vec[pc + 1] as usize;
                vec[pos] = INPUT;
                pc + self.get_step()
            }
            // Third one is always position
            Add { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let pos = vec[pc + 3] as usize;
                vec[pos] = v1 + v2;
                pc + self.get_step()
            }
            Mult { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let pos = vec[pc + 3] as usize;
                vec[pos as usize] = v1 * v2;
                pc + self.get_step()
            }
        }
    }
}

impl From<i32> for Instruction {
    // We can get 2 digits or more
    fn from(item: i32) -> Instruction {
        use Mode::*;
        let str_item = item.to_string();

        let op_code = str_item
            .chars()
            .rev()
            .take(2)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        // Take everything but the last 2 chars

        let modes_vec: Vec<Mode> = str_item
            .chars()
            .rev()
            .skip(2)
            .map(|x| Mode::from(x))
            .collect();

        match op_code {
            99 => Instruction::Halt,
            1 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(3, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1], modes_v[2]];
                Instruction::Add { modes }
            }
            2 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(3, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1], modes_v[2]];
                Instruction::Mult { modes }
            }
            3 => Instruction::Save,
            4 => {
                let modes = modes_vec
                    .into_iter()
                    .pad_using(1, |_| Position)
                    .nth(0)
                    .unwrap();
                Instruction::Output { modes }
            }
            5 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(2, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1]];
                Instruction::JumpIfTrue { modes }
            }
            6 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(2, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1]];
                Instruction::JumpIfFalse { modes }
            }
            7 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(3, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1], modes_v[2]];
                Instruction::LessThan { modes }
            }
            8 => {
                let modes_v: Vec<Mode> = modes_vec.into_iter().pad_using(3, |_| Position).collect();
                let modes = [modes_v[0], modes_v[1], modes_v[2]];
                Instruction::Equals { modes }
            }
            _ => panic!("Unexpected optcode {}", op_code),
        }
    }
}
fn main() {
    let mut inp: Vec<i32> = include_str!("day_05_data.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut pc = 0;

    loop {
        // Parse one instruction

        let instruction: Instruction = inp[pc].into();

        pc = instruction.apply(&mut inp, pc);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::Mode::*;
    use super::*;
    #[test]
    fn test_ins() {
        assert_eq!(Instruction::from(99), Instruction::Halt);
        assert_eq!(
            Instruction::from(1002),
            Instruction::Mult {
                modes: [Position, Immediate, Position]
            }
        );
        assert_eq!(
            Instruction::from(1001),
            Instruction::Add {
                modes: [Position, Immediate, Position]
            }
        );
        assert_eq!(Instruction::from(3), Instruction::Save);
        assert_eq!(
            Instruction::from(4),
            Instruction::Output { modes: Position }
        );
    }
}
