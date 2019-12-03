fn fuel_for_fuel(mass: i32) -> i32 {
    let fuel_est = (mass / 3) - 2;
    if fuel_est <= 0 {
        0
    } else {
        fuel_est + fuel_for_fuel(fuel_est)
    }
}

fn main() {
    let numbers: i32 = include_str!("day_01_data.txt")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .map(fuel_for_fuel)
        .sum();
    dbg!(&numbers);
}
