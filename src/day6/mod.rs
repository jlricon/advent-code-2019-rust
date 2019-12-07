use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
#[derive(Debug)]
pub struct OrbitPair {
    orbited: String,
    orbiter: String,
}

fn count_orbits(orbiter: &str, graph: &HashMap<&str, &str>) -> u32 {
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
    match orbiter {
        "COM" => v,
        _ => {
            let parent = *graph.get(orbiter).unwrap();
            let v2 = v.into_iter().chain(std::iter::once(parent)).collect();
            get_hierar(parent, graph, v2)
        }
    }
}

fn get_jumps_to(santa_and_me: &HashMap<&str, Vec<&str>>, intersec_spot: &str, who: &str) -> usize {
    santa_and_me
        .get(who)
        .unwrap()
        .iter()
        .enumerate()
        .take_while(|v| *v.1 != intersec_spot)
        .map(|v| v.0)
        .max()
        .unwrap()
}

pub fn get_jumps(orbits: Vec<OrbitPair>) -> (usize, usize) {
    let graph: HashMap<&str, &str> = orbits
        .iter()
        .map(|orbit| (orbit.orbiter.as_str(), orbit.orbited.as_str()))
        .collect();
    let norbits: u32 = graph.keys().map(|x| count_orbits(x, &graph)).sum();

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
    let my_planets: HashSet<&&str> = HashSet::from_iter(santa_and_me.get("SAN").unwrap().iter());

    let santa_planets: HashSet<&&str> = HashSet::from_iter(santa_and_me.get("YOU").unwrap().iter());
    let intersec: HashSet<&&&str> =
        HashSet::intersection(&my_planets, &santa_planets).collect::<HashSet<&&&str>>();
    // Find hops to first common planet
    let intersec_spot = santa_and_me
        .get("YOU")
        .unwrap()
        .iter()
        .filter(|x| intersec.contains(x))
        .nth(0)
        .unwrap();

    let me_to_inter = get_jumps_to(&santa_and_me, intersec_spot, "YOU");
    let san_to_inter = get_jumps_to(&santa_and_me, intersec_spot, "SAN");
    (me_to_inter + san_to_inter + 2, norbits as usize)
}

pub fn str_to_orbits(x: &str) -> Vec<OrbitPair> {
    x.lines()
        .map(|x| OrbitPair {
            orbited: x.split(')').nth(0).unwrap().into(),
            orbiter: x.split(')').nth(1).unwrap().into(),
        })
        .collect()
}
