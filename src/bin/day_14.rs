use std::collections::HashMap;

//8 SPJN, 2 LJRB, 1 QMDTJ => 1 TFPRF
#[derive(Debug)]
struct Reaction {
    inputs: HashMap<String, usize>,
    output: String,
    output_amount: usize,
}

impl Reaction {
    fn from_line(x: &str) -> Reaction {
        let mut io = x.split(" => ");
        let input = io.next().unwrap();
        let outputs: Vec<&str> = io.next().unwrap().split(" ").collect();
        let output_amount: usize = outputs[0].parse().unwrap();
        let output = outputs[1].to_owned();
        let inputs: HashMap<String, usize> = input
            .split(", ")
            .map(|v| {
                let parts: Vec<&str> = v.split(" ").collect();
                let n = parts[0].parse::<usize>().unwrap();
                let name = parts[1].to_owned();
                (name, n)
            })
            .collect();
        Reaction {
            inputs,
            output,
            output_amount,
        }
    }
}
type Chemical = String;
type ChemicalToAmount = HashMap<String, usize>;
fn get_ore(
    m: &Chemical,
    map: &HashMap<Chemical, ChemicalToAmount>,
    mut cache: &mut ChemicalToAmount,
) -> usize {
    if m == "ORE" {
        1
    } else {
        let reac = map.get(m).unwrap();

        reac.iter()
            .map(|(k, v)| match cache.get(k) {
                None => {
                    let res = get_ore(k, map, cache);
                    cache.insert(k.to_string(), res);
                    v * res
                }
                Some(val) => v * val,
            })
            .sum()
    }
}
fn main() {
    let inp: Vec<Reaction> = include_str!("day_14_data.txt")
        .lines()
        .map(Reaction::from_line)
        .collect();

    dbg!(part1(inp));
}

fn part1(inp: Vec<Reaction>) -> usize {
    // For each output, the relevant reaction
    let map: HashMap<Chemical, ChemicalToAmount> =
        inp.into_iter().map(|k| (k.output, k.inputs)).collect();
    let mut cache = HashMap::new();
    get_ore(&"FUEL".to_owned(), &map, &mut cache)
}

mod test {
    use super::*;
    #[test]
    fn test_1() {
        //        let inp: Vec<Reaction> = "157 ORE => 5 NZVS
        //165 ORE => 6 DCFZ
        //44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        //12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        //179 ORE => 7 PSHF
        //177 ORE => 5 HKGWZ
        //7 DCFZ, 7 PSHF => 2 XJWVT
        //165 ORE => 2 GPVTF
        //3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        let inp: Vec<Reaction> = "10 ORE => 3 XJWVT\n2 XJWVT => 1 FUEL"
            .lines()
            .map(Reaction::from_line)
            .collect();
        let map: HashMap<Chemical, ChemicalToAmount> =
            inp.into_iter().map(|k| (k.output, k.inputs)).collect();
        let mut cache = HashMap::new();
        let ret = get_ore(&"FUEL".to_owned(), &map, &mut cache);
        assert_eq!(ret, 10);
    }
}
