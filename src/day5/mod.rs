use itertools::{zip, Itertools};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
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
pub enum Instruction {
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
pub struct Outcome {
    pub advance: usize,
    pub output: Option<i64>,
}

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

    pub fn apply(&self, vec: &mut Vec<i64>, pc: usize, input_val: Option<&i64>) -> Outcome {
        use Instruction::*;
        use Mode::*;

        fn get_vals(vec: &Vec<i64>, modes: Vec<Mode>, pc: usize) -> (i64, i64) {
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
                let ret = if v1 != 0 {
                    v2 as usize
                } else {
                    pc + self.get_step()
                };
                Outcome {
                    advance: ret,
                    output: None,
                }
            }
            JumpIfFalse { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let ret = if v1 == 0 {
                    v2 as usize
                } else {
                    pc + self.get_step()
                };
                Outcome {
                    advance: ret,
                    output: None,
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
                Outcome {
                    advance: pc + self.get_step(),
                    output: None,
                }
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
                Outcome {
                    advance: pc + self.get_step(),
                    output: None,
                }
            }
            Halt => Outcome {
                advance: 0,
                output: None,
            },
            Output { modes } => {
                let pos1 = vec[pc + 1] as usize;
                let out = match modes {
                    Position => vec[pos1 as usize],
                    Immediate => pos1 as i64,
                };
                Outcome {
                    advance: pc + self.get_step(),
                    output: Some(out),
                }
            }
            Save => {
                let pos = vec[pc + 1] as usize;
                match input_val {
                    Some(v) => {
                        vec[pos] = *v;
                    }
                    None => panic!("No more inputs!"),
                }

                Outcome {
                    advance: pc + self.get_step(),
                    output: None,
                }
            }
            // Third one is always position
            Add { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let pos = vec[pc + 3] as usize;
                vec[pos] = v1 + v2;

                Outcome {
                    advance: pc + self.get_step(),
                    output: None,
                }
            }
            Mult { modes } => {
                let (v1, v2, ..) = get_vals(&vec, modes.to_vec(), pc);
                let pos = vec[pc + 3] as usize;
                vec[pos] = v1 * v2;
                Outcome {
                    advance: pc + self.get_step(),
                    output: None,
                }
            }
        }
    }
}

impl From<i64> for Instruction {
    // We can get 2 digits or more
    fn from(item: i64) -> Instruction {
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
pub fn run_computer(inp: &Vec<i64>, input_vals: Vec<i64>, pc: usize) -> (Option<i64>, usize) {
    let mut pc = pc;
    let mut input_val_pos = 0;
    let mut input_val = input_vals.iter().nth(input_val_pos);
    let mut local_inp = inp.clone();

    loop {
        // Parse one instruction

        let instruction: Instruction = local_inp[pc].into();
        match instruction {
            Instruction::Halt => {
                dbg!("Halt!");
                return (Option::<i64>::None, pc);
            }
            _ => (),
        };
        let outcome = instruction.apply(&mut local_inp, pc, input_val);

        if instruction == Instruction::Save {
            input_val_pos += 1;
            input_val = input_vals.iter().nth(input_val_pos);
        }
        pc = outcome.advance;
        match outcome.output {
            Some(o) => {
                dbg!(o);
                return (Some(o), pc);
            }
            None => (),
        };
    }
}
pub fn run_five(inp: &Vec<i64>, phase: Vec<i64>) -> Option<i64> {
    let mut out = Some(0);
    let mut pcs = [0, 0, 0, 0, 0];

    loop {
        for (i, pc_i) in zip(&phase, 0..=4) {
            let input_vals = vec![*i, out.unwrap()];
            let ret = run_computer(inp, input_vals, pcs[pc_i]);
            pcs[pc_i] = ret.1;
            if ret.0 == None {
                return out;
            }

            out = ret.0;
        }
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::Mode::*;
    use super::*;
    use std::iter::FromIterator;
    use std::{fs, path};

    #[test]
    fn test_case() {
        let inp: Vec<i64> = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let input_val = Vec::from_iter(std::iter::once(5));
        let (res, _) = run_computer(&inp, input_val, 0);
        assert_eq!(res, Some(999));
        let input_val = Vec::from_iter(std::iter::once(8));
        let (res, _) = run_computer(&inp, input_val, 0);
        assert_eq!(res, Some(1000));
    }
    #[test]
    fn test_case2() {
        let mut pc = 0;
        let path = path::Path::new("./src/bin/day_05_data.txt").canonicalize();
        let val = fs::read_to_string(path.unwrap()).unwrap();
        let mut inp: Vec<i64> = val
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let instruction: Instruction = inp[pc].into();
        //'[3, 225, 1, 225, 6, 6, 1100, 1, 238, 225]
        assert_eq!(instruction, Instruction::Save);

        let outcome = instruction.apply(&mut inp, pc, Some(&5));
        assert_eq!(inp[225], 5);
        pc = outcome.advance;
        let instruction2: Instruction = inp[pc].into();
        assert_eq!(
            instruction2,
            Instruction::Add {
                modes: [Position, Position, Position]
            }
        );
        let outcome2 = instruction2.apply(&mut inp, pc, None);
        assert_eq!(inp[6], 1100 + 5);
        pc = outcome2.advance;
        let instruction3: Instruction = inp[pc].into();
        assert_eq!(
            instruction3,
            Instruction::JumpIfTrue {
                modes: [Immediate, Immediate]
            }
        );
    }
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
    #[test]
    fn test_run1() {
        let inp: Vec<i64> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut out = Some(0);

        let phase = vec![4, 3, 2, 1, 0];

        for i in phase {
            let input_vals = vec![i, out.unwrap()];
            out = run_computer(&inp, input_vals, 0).0;
        }

        assert_eq!(out, Some(43210));
    }

    #[test]
    fn test_run5() {
        let mut inp: Vec<i64> =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        //9 and 0
        let mut pc = 0;
        let instruction: Instruction = inp[pc].into();
        assert_eq!(instruction, Instruction::Save);
        let outcome = instruction.apply(&mut inp, pc, Some(&9));
        assert_eq!(inp[26], 9);
        pc = outcome.advance;
        let instruction: Instruction = inp[pc].into();
        assert_eq!(
            instruction,
            Instruction::Add {
                modes: [Position, Immediate, Position]
            }
        );
        let outcome = instruction.apply(&mut inp, pc, Some(&0));
        assert_eq!(inp[26], 9 - 4);
        pc = outcome.advance;
        let instruction: Instruction = inp[pc].into();
        assert_eq!(instruction, Instruction::Save);
        let outcome = instruction.apply(&mut inp, pc, Some(&0));
        assert_eq!(inp[27], 0);
        pc = outcome.advance;
        let instruction: Instruction = inp[pc].into();
        assert_eq!(
            instruction,
            Instruction::Mult {
                modes: [Position, Immediate, Position]
            }
        );
    }
    #[test]
    fn test_run5_real() {
        let inp: Vec<i64> =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
        let ret = run_five(&inp, vec![9, 8, 7, 6, 5]);
        assert_eq!(ret, Some(139629729));
    }
}
