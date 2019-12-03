fn main() {
    let numbers: i32 = include_str!("day_01_data.txt")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(|x| (x / 3) - 2)
        .sum();
    dbg!(&numbers);
}
