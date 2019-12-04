use itertools::Itertools;

fn is_valid(x: i32) -> bool {
    // Digits in increasing number
    // Group of two but not more
    let chars: Vec<char> = x.to_string().chars().collect();
    let cond1 = chars.windows(2).all(|c| c[0] <= c[1]);
    let cond2 = chars
        .into_iter()
        .group_by(|k| *k)
        .into_iter()
        .map(|(k, v)| v.count())
        .filter(|x| *x == 2)
        .count()
        >= 1;

    cond1 & cond2
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
