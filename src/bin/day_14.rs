use std::collections::HashMap;

//8 SPJN, 2 LJRB, 1 QMDTJ => 1 TFPRF
#[derive(Debug, Clone)]
struct Reaction<'a> {
    inputs: HashMap<Chemical<'a>, usize>,
    output: Chemical<'a>,
    output_amount: usize,
}
const ORE: &str = "ORE";
const TRILLION: usize = 1_000_000_000_000;
impl Reaction<'_> {
    fn from_line(x: &str) -> Reaction {
        let mut io = x.split(" => ");
        let input = io.next().unwrap();
        let outputs: Vec<&str> = io.next().unwrap().split(" ").collect();
        let output_amount: usize = outputs[0].parse().unwrap();
        let output = outputs[1];
        let inputs: HashMap<Chemical, usize> = input
            .split(", ")
            .map(|v| {
                let parts: Vec<&str> = v.split(" ").collect();
                let n = parts[0].parse::<usize>().unwrap();
                let name = parts[1];
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
fn get_ratio_to_make(output: usize, needed: usize) -> usize {
    // Given a needed output and an input->output relation, return the input that makes the output
    // Equal or greater than needed eg. 5A->7B and need 2 B ->5A
    (needed + output - 1) / output
}
type Chemical<'a> = &'a str;
type ChemicalToAmount<'a> = (usize, HashMap<Chemical<'a>, usize>);
fn get_ore(m: Chemical, n: usize, map: &HashMap<Chemical, ChemicalToAmount>) -> usize {
    let mut need = Vec::new();
    let mut store: HashMap<Chemical, usize> = HashMap::new();
    let mut ore_needed = 0;
    need.push((m, n));
    store.insert(ORE, 0);

    while let Some((needed_chemical, needed_amount)) = need.pop() {
        //        println!(
        //            "We need to make {} units of {}",
        //            needed_amount, needed_chemical
        //        );
        //        println!("We have in the store {:?}", &store);
        // Do we need to make more?
        let available_in_store = match store.get(needed_chemical) {
            // We found some
            Some(stored_amount) => *stored_amount,
            None => 0,
        };
        if available_in_store >= needed_amount {
            //            println!(
            //                "The store has {}, we just need {}",
            //                available_in_store, needed_amount
            //            );
            store.insert(needed_chemical, available_in_store - needed_amount);
        } else {
            // Take what we need from the store, check how much we still need
            let still_needed = needed_amount - available_in_store;
            store.insert(needed_chemical, 0);
            // Chemicals needed to make the output we need
            let chemicals_to_make = map.get(needed_chemical).unwrap();
            // With these chemicals we need to make this amount of output
            let output_amount = chemicals_to_make.0;

            let ratio = get_ratio_to_make(output_amount, still_needed);
            //            println!("To do so we need {}x{:?}", ratio, chemicals_to_make);
            chemicals_to_make.1.iter().for_each(|(chem, input)| {
                // Add to the pile of needs
                // We need to make `still_needed`
                // e.g. if the reaction is 2(input)A->3(output)B and we need 5(still_needed)B
                // Then we really need 4A to get 6B. So

                let real_input = *input * ratio;
                if *chem == ORE {
                    ore_needed += real_input;
                } else {
                    need.push((*chem, real_input));
                }
            });
            let leftover = output_amount * ratio - still_needed;
            //            println!(
            //                "And we insert a leftover amount of {} back to the store",
            //                leftover
            //            );
            *store.entry(needed_chemical).or_insert(0) += leftover;
        }
        //        println!("We now need {:?}", need);
        //        print!("{}", '\n')
    }
    //    println!("{:?}", &store);
    ore_needed
}
fn main() {
    let inp: Vec<Reaction> = include_str!("day_14_data.txt")
        .lines()
        .map(Reaction::from_line)
        .collect();
    let inp2 = inp.clone();
    dbg!(part1(inp));
    dbg!(part2(inp2));
}

fn part1(inp: Vec<Reaction>) -> usize {
    // For each output, the relevant reaction
    let map: HashMap<Chemical, ChemicalToAmount> = inp
        .into_iter()
        .map(|k| (k.output, (k.output_amount, k.inputs)))
        .collect();

    get_ore(&"FUEL".to_owned(), 1, &map)
}
fn part2(inp: Vec<Reaction>) -> usize {
    // For each output, the relevant reaction
    let map: HashMap<Chemical, ChemicalToAmount> = inp
        .into_iter()
        .map(|k| (k.output, (k.output_amount, k.inputs)))
        .collect();
    let mut max_ore = 2800000;
    loop {
        let res = get_ore(&"FUEL".to_owned(), max_ore, &map);
        if res == TRILLION {
            break max_ore;
        } else if res > TRILLION {
            break max_ore - 1;
        }
        max_ore += 1;
        //        dbg!(res);
    }
}
mod test {
    use super::*;
    #[test]
    fn test_ceil() {
        let ret = get_ratio_to_make(1, 1);
        assert_eq!(ret, 1);
        let ret = get_ratio_to_make(3, 7);
        assert_eq!(ret, 3);
        let ret = get_ratio_to_make(3, 5);
        assert_eq!(ret, 2);
        let ret = get_ratio_to_make(5, 10);
        assert_eq!(ret, 2);
    }
    #[test]
    fn test_0() {
        let inp: Vec<Reaction> = "1 ORE => 3 E\n3 E => 2 A\n7 A, 1 E => 1 FUEL"
            .lines()
            .map(Reaction::from_line)
            .collect();
        let map: HashMap<Chemical, ChemicalToAmount> = inp
            .into_iter()
            .map(|k| (k.output, (k.output_amount, k.inputs)))
            .collect();

        let ret = get_ore(&"FUEL".to_owned(), 1, &map);
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
    #[test]
    fn test_2() {
        let inp: Vec<Reaction> = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            .lines()
            .map(Reaction::from_line)
            .collect();
        let map: HashMap<Chemical, ChemicalToAmount> = inp
            .into_iter()
            .map(|k| (k.output, (k.output_amount, k.inputs)))
            .collect();

        let ret = get_ore(&"FUEL".to_owned(), 1, &map);
        assert_eq!(ret, 165);
    }
}
