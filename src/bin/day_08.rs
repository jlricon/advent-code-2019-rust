use itertools::Itertools;
use std::str::Chars;

const PER_LAYER: usize = 25 * 6;

fn part1(layers: &[String]) {
    let zeros_min: Vec<usize> = layers
        .iter()
        .map(|layer| layer.chars().filter(|c| *c == '0').count())
        .collect();
    let max_pos = zeros_min.iter().enumerate().min_by_key(|c| c.1).unwrap().0;
    dbg!(max_pos);
    let max_layer: &String = &layers[max_pos];

    let counts: usize = max_layer
        .chars()
        .filter(|c| (*c == '1') | (*c == '2'))
        .sorted()
        .group_by(|k| *k)
        .into_iter()
        .map(|(_, g)| g.count())
        .product();
    dbg!(counts);
}
fn part2(layers: &[String]) {
    // Zip all the layers
    let layer_chars: Vec<Chars> = layers.iter().map(|s| s.chars()).collect();
    let mut stacked = ['2'; PER_LAYER];
    layer_chars.into_iter().for_each(|layer| {
        layer.into_iter().enumerate().for_each(|(pos, c)| {
            if stacked[pos] == '2' {
                stacked[pos] = c;
            }
        })
    });

    let st = stacked.to_vec();
    show(st, 25)
}
fn show(img: Vec<char>, w: usize) {
    img.chunks(w).for_each(|l| {
        let line: String = l
            .iter()
            .map(|p| match p {
                '1' => '#',
                _ => ' ',
            })
            .collect();
        println!("{}", line);
    })
}
fn main() {
    let inp: &str = include_str!("day_08_data.txt").lines().nth(0).unwrap();
    let layers: Vec<String> = inp
        .chars()
        .chunks(PER_LAYER)
        .into_iter()
        .map(|c| c.collect())
        .collect();
    part1(&layers);
    part2(&layers)
}
