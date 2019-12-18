use itertools::Itertools;
enum Ops {
    Plus,
    Minus,
    None,
}
const BASE_PATTERN: [Ops; 4] = [Ops::None, Ops::Plus, Ops::None, Ops::Minus];
fn input_list_times_pattern(x: &Vec<u32>, pos: usize) -> u32 {
    let pattern = BASE_PATTERN
        .into_iter()
        .chain(BASE_PATTERN.iter())
        .flat_map(|v| std::iter::repeat(v).skip(pos).take(pos))
        .cycle()
        .skip(1);

    x.into_iter()
        .zip(pattern)
        .filter_map(|(l, p)| match p {
            Ops::None => None,
            Ops::Plus => Some(*l as i32),
            Ops::Minus => Some(-(*l as i32)),
        })
        .sum::<i32>()
        .abs() as u32
        % 10
}
fn apply_1_phase(inp: &Vec<u32>, n: usize) -> Vec<u32> {
    (1..=inp.len())
        .into_iter()
        .map(|n| input_list_times_pattern(inp, n))
        .collect()
}
fn apply_n_phases(x: Vec<u32>, n: usize) -> Vec<u32> {
    (1..=n)
        .fold(x, |acc, n| apply_1_phase(&acc, n))
        .into_iter()
        .collect()
}
fn part1() {
    let inp = include_str!("day_16_data.txt")
        .trim()
        .chars()
        .map(|v| v as u32 - '0' as u32)
        .collect();

    let list = apply_n_phases(inp, 100).iter().take(8).join("");
    dbg!(list);
}
fn main() {
    let inp: Vec<u32> = include_str!("day_16_data.txt")
        .trim()
        .chars()
        .map(|v| v as u32 - '0' as u32)
        .collect();
    let skipped_list = get_top_8(inp);

    dbg!(skipped_list);
}

fn get_top_8(inp: Vec<u32>) -> usize {
    let inp_len = inp.len();

    let offset = (&inp)
        .into_iter()
        .take(7)
        .map(|v| (*v as u8 + '0' as u8) as char)
        .collect::<String>()
        .parse()
        .unwrap();

    let mut digits: Vec<u32> = inp
        .into_iter()
        .cycle()
        .take(10000 * inp_len)
        .skip(offset)
        .collect();

    // https://github.com/jackmott/advent2019/blob/master/day_16/src/main.rs#L4
    for _ in 0..100 {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }
    digits
        .iter()
        .take(8)
        .map(|v| (*v as u8 + '0' as u8) as char)
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn test_1() {
        let inp: Vec<u32> = "12345678"
            .chars()
            .map(|v| v as u32 - '0' as u32)
            .collect_vec();
        let o1 = input_list_times_pattern(&inp, 1);
        assert_eq!(o1, 4);
        let o1 = input_list_times_pattern(&inp, 2);
        assert_eq!(o1, 8);
        let o1 = input_list_times_pattern(&inp, 3);
        assert_eq!(o1, 2);
        let full: Vec<_> = (1..=inp.len())
            .map(|n| input_list_times_pattern(&inp, n))
            .collect();
        assert_eq!(full, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let one_phase = apply_n_phases(inp.clone(), 1);
        assert_eq!(one_phase, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        let two_phases = apply_n_phases(inp, 2);
        assert_eq!(two_phases, vec![3, 4, 0, 4, 0, 4, 3, 8])
    }
    #[test]
    fn test_2() {
        let inp: Vec<u32> = "80871224585914546619083218645595"
            .chars()
            .map(|v| v as u32 - '0' as u32)
            .collect_vec();
        let two_phases: Vec<u32> = apply_n_phases(inp, 100).into_iter().take(8).collect();
        assert_eq!(two_phases, vec![2, 4, 1, 7, 6, 1, 7, 6])
    }

    #[test]
    fn test_5() {
        let inp: Vec<u32> = "03036732577212944063491565474664"
            .chars()
            .map(|v| v as u32 - '0' as u32)
            .collect_vec();
        let skipped_list = get_top_8(inp);
        assert_eq!(skipped_list, 84462026);
    }
}
