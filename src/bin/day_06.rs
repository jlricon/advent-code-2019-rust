#[derive(Debug)]
struct OrbitPair {
    orbited: String,
    orbiter: String,
}
use std::collections::HashMap;
fn count_orbits(orbiter: &str, graph: &HashMap<String, String>) -> u32 {
    if orbiter == "COM" {
        0
    } else {
        1 + count_orbits(graph.get(orbiter).unwrap(), graph)
    }
}
fn main() {
    let orbits: Vec<OrbitPair> = include_str!("day_06_data.txt")
        .lines()
        .map(|x| OrbitPair {
            orbited: x.split(')').nth(0).unwrap().into(),
            orbiter: x.split(')').nth(1).unwrap().into(),
        })
        .collect();
    let mut graph: HashMap<String, String> = HashMap::new();
    orbits.into_iter().for_each(|orbit| {
        graph.insert(orbit.orbiter, orbit.orbited);
    });
    let res: u32 = graph.keys().map(|x| count_orbits(x, &graph)).sum();
    dbg!(res);
}
