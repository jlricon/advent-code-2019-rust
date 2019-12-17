const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];
fn input_list_times_pattern<'a>(x: impl Iterator<Item = &'a u32>, pos: usize) -> u32 {
    let pattern = BASE_PATTERN
        .iter()
        .skip({
            if pos == 1 {
                1
            } else {
                0
            }
        })
        .cycle();

    x.zip(pattern)
        .map(|(l, p)| {
            dbg!(l, p);
            *l as i32 * p
        })
        .sum::<i32>()
        .abs() as u32
        % 10
}
fn main() {
    //    let inp = include_str!("day_16_data.txt");

    let list = input_list_times_pattern([0, 1, 2].iter(), 1);
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
        let o1 = input_list_times_pattern(inp.iter(), 1);
        assert_eq!(o1, 4);
        let o1 = input_list_times_pattern(inp.iter(), 2);
        assert_eq!(o1, 8);
    }
}
