#[derive(Debug)]
struct OrbitPair {
    orbited: String,
    orbiter: String,
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn count_orbits(orbiter: &str, graph: &HashMap<String, String>) -> u32 {
    if orbiter == "COM" {
        0
    } else {
        1 + count_orbits(graph.get(orbiter).unwrap(), graph)
    }
}
fn get_hierar<'a>(
    orbiter: &'a str,
    graph: &HashMap<&'a str, &'a str>,
    v: Vec<&'a str>,
) -> Vec<&'a str> {
    if orbiter == "COM" {
        v
    } else {
        let mut v2 = v.clone();
        let parent = graph.get(orbiter).unwrap();
        v2.push(*parent);
        get_hierar(parent, graph, v2)
    }
}
fn get_jumps_to(santa_and_me: &HashMap<&str, Vec<&str>>, intersec_spot: &str, who: &str) -> usize {
    santa_and_me
        .get(who)
        .unwrap()
        .into_iter()
        .enumerate()
        .take_while(|v| *v.1 != intersec_spot)
        .map(|v| v.0)
        .max()
        .unwrap()
}
fn str_to_orbits(x: &str) -> Vec<OrbitPair> {
    x.lines()
        .map(|x| OrbitPair {
            orbited: x.split(')').nth(0).unwrap().into(),
            orbiter: x.split(')').nth(1).unwrap().into(),
        })
        .collect()
}
fn main() {
    let orbits: Vec<OrbitPair> = str_to_orbits(include_str!("day_06_data.txt"));

    let res = get_jumps(orbits);
    dbg!(res);
}

fn get_jumps(orbits: Vec<OrbitPair>) -> usize {
    let mut graph: HashMap<&str, &str> = HashMap::new();
    orbits.iter().for_each(|orbit| {
        graph.insert(&orbit.orbiter, &orbit.orbited);
    });

    let santa_and_me: HashMap<&str, Vec<&str>> = orbits
        .iter()
        .filter(|x| (&x.orbiter == "SAN") | (&x.orbiter == "YOU"))
        .map(|x| {
            (
                x.orbiter.as_str(),
                get_hierar(&x.orbiter, &graph, Vec::new()),
            )
        })
        .collect();
    let my_planets: HashSet<&&str> =
        HashSet::from_iter(santa_and_me.get("SAN").unwrap().into_iter());
    let santa_planets: HashSet<&&str> =
        HashSet::from_iter(santa_and_me.get("YOU").unwrap().into_iter());
    let intersec: HashSet<&&&str> =
        HashSet::intersection(&my_planets, &santa_planets).collect::<HashSet<&&&str>>();
    // Find hops to first common planet
    let intersec_spot = santa_and_me
        .get("YOU")
        .unwrap()
        .into_iter()
        .filter(|x| intersec.contains(x))
        .nth(0)
        .unwrap();

    let me_to_inter = get_jumps_to(&santa_and_me, intersec_spot, "YOU");
    let san_to_inter = get_jumps_to(&santa_and_me, intersec_spot, "SAN");
    me_to_inter + san_to_inter + 2
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
        assert_eq!(jumps, 4);
    }
}
