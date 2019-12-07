use advent_code_2019_rust::day6::{get_jumps, str_to_orbits, OrbitPair};

fn main() {
    let orbits: Vec<OrbitPair> = str_to_orbits(include_str!("day_06_data.txt"));
    let res = get_jumps(orbits);
    dbg!(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ins() {
        let tst = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let orbits = str_to_orbits(tst);
        let jumps = get_jumps(orbits);
        assert_eq!(jumps, (4, 54));
    }
}
