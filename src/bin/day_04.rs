use itertools::EitherOrBoth;
use itertools::Itertools;
use std::collections::HashSet;

fn is_valid(x: i32) -> bool {
    // Digits in increasing number
    // Group of two but not more
    let stn: Vec<char> = format!("{}", x).chars().collect();

    // Digits in increasing number
    let mut group_len = 1;
    let mut group_lens = HashSet::new();
    let mut lastn = &'a';
    let cond1 = stn
        .iter()
        .zip_longest(stn.iter().skip(1))
        .fold(true, |prev, x| match x {
            EitherOrBoth::Both(x, y) => {
                if x == y {
                    group_len += 1;
                } else {
                    group_lens.insert(group_len);
                    group_len = 1;
                };
                lastn = y;
                (y >= x) & prev
            }
            EitherOrBoth::Left(x) => {
                if x == lastn {
                    group_lens.insert(group_len);
                };
                prev
            }
            _ => unreachable!(),
        });
    // Group of two but not more
    let cond2 = group_lens.contains(&2);

    if cond1 & cond2 {
        true
    } else {
        false
    }
}
fn main() {
    // It is a six-digit number.
    //The value is within the range given in your puzzle input.
    //Two adjacent digits are the same (like 22 in 122345).
    //Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

    let min = 172851;
    let max = 675869;
    let res = (min..max)
        .map(|x| is_valid(x))
        .filter(|x| *x)
        .fold(0, |a, _| a + 1);

    dbg!(res);
}
