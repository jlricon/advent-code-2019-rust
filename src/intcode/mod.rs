use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

type Instr = i64;

trait InstrOps {
    fn is_immediate_param(&self, nth: usize) -> bool;
}

impl InstrOps for Instr {
    fn is_immediate_param(&self, nth: usize) -> bool {
        (self / (10_i64.pow(nth as u32 + 1))) % 10 == 1
    }
}

#[test]
fn test_immediate() {
    assert_eq!(true, 1101.is_immediate_param(1));
    assert_eq!(true, 1101.is_immediate_param(2));
    assert_eq!(false, 1101.is_immediate_param(3));
    assert_eq!(false, 1001.is_immediate_param(1));
    assert_eq!(true, 1001.is_immediate_param(2));
}

#[derive(Debug, Clone)]
pub enum OpResult {
    Unit,
    End,
    Error,
    Print(i64),
}

pub struct IntCodeProg {
    instrs: Vec<Instr>,
    pc: usize,
    inputs: Vec<i64>,
}

impl IntCodeProg {
    pub fn from_file(path: &str) -> Result<IntCodeProg> {
        let f = File::open(path)?;
        let f = BufReader::new(f);

        let instrs: Vec<Instr> = f
            .split(b',')
            .filter_map(|token| token.ok())
            .filter_map(|token| String::from_utf8(token).ok())
            .filter_map(|num| i64::from_str_radix(&num.trim(), 10).ok())
            .collect();

        return Ok(IntCodeProg {
            instrs: instrs,
            pc: 0,
            inputs: Vec::new(),
        });
    }

    fn opcode(&self) -> i64 {
        self.instrs[self.pc] % 100
    }

    fn param(&self, nth: usize) -> i64 {
        match self.instrs[self.pc].is_immediate_param(nth) {
            true => self.instrs[self.pc + nth],
            false => self.instrs[self.instrs[self.pc + nth] as usize],
        }
    }

    fn set(&mut self, nth: usize, res: i64) -> OpResult {
        let ptr = self.instrs[self.pc + nth] as usize;
        self.instrs[ptr] = res;
        OpResult::Unit
    }

    pub fn add_input(&mut self, v: i64) {
        self.inputs.insert(0, v);
    }

    pub fn push_input(&mut self, v: i64) {
        self.inputs.push(v);
    }

    fn eval_next(&mut self) -> Option<OpResult> {
        let (result, next_pc) = match self.opcode() {
            1 => (self.set(3, self.param(1) + self.param(2)), self.pc + 4),
            2 => (self.set(3, self.param(1) * self.param(2)), self.pc + 4),
            3 => match self.inputs.pop() {
                Some(v) => (self.set(1, v), self.pc + 2),
                None => (OpResult::Error, self.pc),
            },
            4 => (OpResult::Print(self.param(1)), self.pc + 2),
            5 => (
                OpResult::Unit,
                match self.param(1) != 0 {
                    true => self.param(2) as usize,
                    false => self.pc + 3,
                },
            ),
            6 => (
                OpResult::Unit,
                match self.param(1) == 0 {
                    true => self.param(2) as usize,
                    false => self.pc + 3,
                },
            ),
            7 => (
                self.set(
                    3,
                    match self.param(1) < self.param(2) {
                        true => 1,
                        false => 0,
                    },
                ),
                self.pc + 4,
            ),
            8 => (
                self.set(
                    3,
                    match self.param(1) == self.param(2) {
                        true => 1,
                        false => 0,
                    },
                ),
                self.pc + 4,
            ),
            99 => (OpResult::End, 0),
            _ => (OpResult::Error, 0),
        };

        match result {
            OpResult::End | OpResult::Error => (),
            _ => self.pc = next_pc,
        }

        Some(result)
    }
}

impl Iterator for IntCodeProg {
    type Item = OpResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.eval_next()
    }
}
