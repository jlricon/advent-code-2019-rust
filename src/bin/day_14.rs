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
fn get_input_to_make(input: usize, output: usize, needed: usize) -> usize {
    // Given a needed output and an input->output relation, return the input that makes the output
    // Equal or greater than needed eg. 5A->7B and need 2 B ->5A
    input * (needed / output + needed % output)
}
type Chemical = String;
type ChemicalToAmount = (usize, HashMap<String, usize>);
fn get_ore(m: &Chemical, n: usize, map: &HashMap<Chemical, ChemicalToAmount>) -> usize {
    let mut need = Vec::new();
    let mut store: HashMap<&Chemical, usize> = HashMap::new();
    let mut ore_needed = 0;
    need.push((m, n));
    while let Some((needed_chemical, needed_amount)) = need.pop() {
        println!(
            "We need to make {} units of {}",
            needed_amount, needed_chemical
        );
        // Do we need to make more?
        let available_in_store = match store.get(needed_chemical) {
            // We found some
            Some(stored_amount) => *stored_amount,
            None => 0,
        };
        if available_in_store >= needed_amount {
            println!(
                "The store has {}, we just need {}",
                available_in_store, needed_amount
            );
            store.insert(needed_chemical, available_in_store - needed_amount);
        } else {
            // Take what we need from the store, check how much we still need
            let still_needed = needed_amount - available_in_store;
            store.insert(needed_chemical, 0);
            let chemicals_to_make = map.get(needed_chemical).unwrap();
            println!("To do so we need {:?}", chemicals_to_make);
            chemicals_to_make.1.iter().for_each(|(chem, input)| {
                // Add to the pile of needs
                // We need to make `still_needed`
                // e.g. if the reaction is 2(input)A->3(output)B and we need 5(still_needed)B
                // Then we really need 4A to get 6B. So

                if chem == "ORE" {
                    ore_needed += input * still_needed;
                } else {
                    need.push((chem, input * still_needed));
                }
            })
        }
    }
    ore_needed
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
    let map: HashMap<Chemical, ChemicalToAmount> = inp
        .into_iter()
        .map(|k| (k.output, (k.output_amount, k.inputs)))
        .collect();

    get_ore(&"FUEL".to_owned(), 1, &map)
}

mod test {
    use super::*;
    #[test]
    fn test_ceil() {
        let ret = get_input_to_make(1, 1, 1);
        assert_eq!(ret, 1);
        let ret = get_input_to_make(1, 2, 1);
        assert_eq!(ret, 1);
        let ret = get_input_to_make(1, 2, 2);
        assert_eq!(ret, 1);
        let ret = get_input_to_make(2, 2, 1);
        assert_eq!(ret, 2);
        let ret = get_input_to_make(2, 2, 2);
        assert_eq!(ret, 2);
        let ret = get_input_to_make(5, 7, 2);
        assert_eq!(ret, 5);
    }
    #[test]
    fn test_1() {
        let inp: Vec<Reaction> = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            .lines()
            .map(Reaction::from_line)
            .collect();
        let map: HashMap<Chemical, ChemicalToAmount> = inp
            .into_iter()
            .map(|k| (k.output, (k.output_amount, k.inputs)))
            .collect();

        let ret = get_ore(&"FUEL".to_owned(), 1, &map);
        assert_eq!(ret, 31);
    }
}
