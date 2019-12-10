//use advent_code_2019_rust::intcode::{IntCodeProg, OpResult};
//use std::cell::RefCell;
//use std::io::{Error, ErrorKind, Result};
//use std::rc::Rc;
//
//fn perms_rec(res: &mut Vec<Vec<i64>>, vec: &mut Vec<i64>, from: usize) {
//    if from == vec.len() {
//        res.push(vec.clone());
//    } else {
//        for i in from..vec.len() {
//            vec.swap(from, i);
//            perms_rec(res, vec, from + 1);
//            vec.swap(from, i);
//        }
//    }
//}
//
//fn perms(n: i64) -> Vec<Vec<i64>> {
//    let mut res: Vec<Vec<i64>> = Vec::new();
//    let mut cur: Vec<i64> = (0..n + 1).into_iter().collect();
//
//    perms_rec(&mut res, &mut cur, 0);
//
//    res
//}
//
//#[test]
//fn test_perms() {
//    println!("{:?}", perms(2));
//    println!("{:?}", perms(2).len());
//}
//
//fn eval(path: &str, setting: i64, signal: i64) -> Result<i64> {
//    let mut prog = IntCodeProg::from_file(path)?;
//    prog.push_input(signal);
//    prog.push_input(setting);
//
//    let last = prog
//        .take_while(|result| match result {
//            OpResult::Error => panic!("Error"),
//            OpResult::End => false,
//            _ => true,
//        })
//        .filter(|result| match result {
//            OpResult::Print(_) => true,
//            _ => false,
//        })
//        .last();
//
//    match last {
//        Some(OpResult::Print(val)) => Ok(val),
//        _ => Err(Error::new(ErrorKind::Other, "Can not evaluate expr.")),
//    }
//}
//
//fn eval_batch(path: &str, settings: &Vec<i64>) -> Result<i64> {
//    let mut input: i64 = 0;
//
//    for setting in settings {
//        input = eval(path, *setting, input)?;
//    }
//
//    Ok(input)
//}
//
//fn eval_batch_loop(path: &str, settings: &Vec<i64>) -> Result<i64> {
//    let mut amplifiers: Vec<Rc<RefCell<IntCodeProg>>> = Vec::new();
//    for setting in settings {
//        let mut amplifier = IntCodeProg::from_file(path)?;
//        amplifier.add_input(setting + 5);
//
//        amplifiers.push(Rc::new(RefCell::new(amplifier)));
//    }
//
//    let mut input: i64 = 0;
//
//    for rc in amplifiers.into_iter().cycle() {
//        let mut amplifier = rc.borrow_mut();
//
//        amplifier.add_input(input);
//        loop {
//            match amplifier.next() {
//                Some(OpResult::Print(output)) => {
//                    input = output;
//                    break;
//                }
//                Some(OpResult::End) => return Ok(input),
//                Some(OpResult::Error) | None => panic!("error"),
//                _ => continue,
//            }
//        }
//    }
//
//    Ok(input)
//}
//
//pub fn solve1(path: &str) -> Result<i64> {
//    let mut max: i64 = 0;
//    for settings in perms(4) {
//        max = max.max(eval_batch(path, &settings)?);
//    }
//    Ok(max)
//}
//
//pub fn solve2(path: &str) -> Result<i64> {
//    let mut max: i64 = 0;
//    for settings in perms(4) {
//        max = max.max(eval_batch_loop(path, &settings)?);
//    }
//    Ok(max)
//}
//fn main() {
//    let ret = solve2("gits/advent-code-2019-rust/src/bin/day_07_data.txt").unwrap();
//    dbg!(ret);
//}
