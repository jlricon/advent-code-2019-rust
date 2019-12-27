use im::Vector;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::ToPrimitive;

#[derive(Clone, Debug)]
enum Moves {
    Deal,
    DealInc(usize),
    Cut(i32),
}
#[derive(Debug)]
struct Polynomial(i128, i128, i128);
impl Polynomial {
    fn get_val(&self, x: i128) -> i128 {
        (self.0 * x + self.1) % self.2
    }
    fn get_modpow(&self, exponent: usize) -> Polynomial {
        self.modpow(self.0, self.1, exponent as i128)
    }
    fn modpow(&self, a: i128, b: i128, exponent: i128) -> Polynomial {
        let n = self.2;
        if exponent == 0 {
            Polynomial(1, 0, self.2)
        } else if (exponent % 2) == 0 {
            self.modpow(a * a % n, (a * b + b) % n, exponent / 2)
        } else {
            let poly = self.modpow(a, b, exponent - 1);
            Polynomial(a * poly.0 % n, (a * poly.1 + b) % n, self.2)
        }
    }
}
impl Moves {
    fn apply(&self, it: Vector<u32>) -> Vector<u32> {
        use Moves::*;
        match self {
            Deal => deal_into_new(it),
            DealInc(i) => deal_with_increment(it, *i),
            Cut(i) => cut_n_cards(it, *i),
        }
    }
    fn apply_poly(&self, it: Polynomial) -> Polynomial {
        use Moves::*;
        match self {
            Deal => deal_into_new_poly(it),
            DealInc(i) => deal_inc_poly(it, *i),
            Cut(i) => cut_n_cards_poly(it, *i as i128),
        }
    }
}

fn get_ops(inp: &str) -> Vector<Moves> {
    inp.lines()
        .map(|l| match l {
            "deal into new stack" => Moves::Deal,
            _ if l.starts_with("cut") => l
                .split_ascii_whitespace()
                .nth(1)
                .map(|v| Moves::Cut(v.parse().unwrap()))
                .unwrap(),
            _ if l.starts_with("deal with") => l
                .split_ascii_whitespace()
                .nth(3)
                .map(|v| Moves::DealInc(v.parse().unwrap()))
                .unwrap(),
            _ => panic!(),
        })
        .collect()
}
fn deal_into_new(it: Vector<u32>) -> Vector<u32> {
    it.into_iter()
        .collect::<Vec<u32>>()
        .into_iter()
        .rev()
        .collect()
}
fn deal_into_new_poly(p: Polynomial) -> Polynomial {
    Polynomial(-p.0, p.2 - p.1 - 1, p.2)
}
fn cut_n_cards_pos(it: Vector<u32>, n: usize) -> Vector<u32> {
    let top_n = it.clone().into_iter().take(n);
    it.into_iter().skip(n).chain(top_n).collect()
}
fn cut_n_cards_neg(it: Vector<u32>, n: usize) -> Vector<u32> {
    let top_n = it.clone().into_iter().rev().take(n).rev();
    top_n.into_iter().chain(it.take(it.len() - n)).collect()
}
fn cut_n_cards(it: Vector<u32>, n: i32) -> Vector<u32> {
    if n < 0 {
        cut_n_cards_neg(it, (-n) as usize)
    } else {
        cut_n_cards_pos(it, n as usize)
    }
}
fn cut_n_cards_poly(p: Polynomial, n: i128) -> Polynomial {
    Polynomial(p.0, (p.1 + n) % p.2, p.2)
}
fn deal_with_increment(it: Vector<u32>, n: usize) -> Vector<u32> {
    // We need the entire deck.
    // To go back to the origin we'll be doing mod

    let cycler = (0..it.len()).cycle().step_by(n).take(it.len());

    let mut return_vec: Vector<u32> = vec![0; it.len()].into_iter().collect();
    let cyclable = it.into_iter().collect::<Vec<u32>>().into_iter().cycle();

    cyclable.zip(cycler).for_each(|(el, pos)| {
        return_vec[pos] = el;
    });

    return_vec
}
fn deal_inc_poly(p: Polynomial, n: usize) -> Polynomial {
    let pow = BigInt::modpow(&n.into(), &(p.2 - 2).into(), &p.2.into())
        .to_i128()
        .unwrap();
    Polynomial((p.0 * pow) % p.2, (p.1 * pow) % p.2, p.2)
}
fn apply_ops(baseline: Vector<u32>, ops: Vector<Moves>) -> Vector<u32> {
    ops.iter().fold(baseline, |acc, x| x.apply(acc))
}
fn apply_polys(baseline: Polynomial, ops: Vector<Moves>) -> Polynomial {
    ops.iter().fold(baseline, |acc, x| x.apply_poly(acc))
}
//fn part1(){

//}
fn main() {
    let inp = include_str!("day_22_data.txt");
    let cards: Vector<Moves> = get_ops(inp);

    let cards_rev: Vector<Moves> = get_ops(inp)
        .into_iter()
        .rev()
        .collect::<Vec<Moves>>()
        .into_iter()
        .collect();
    let pos = 2020;
    let L = 119315717514047;
    let N = 101741582076661;
    let applied_polys = apply_polys(Polynomial(1, 0, L), cards_rev);
    let modpowed_poly = applied_polys.get_modpow(N);

    // Part 1
    //    let baseline = (0..=10006).collect::<Vector<u32>>();
    //    let cards = get_ops(inp);
    //    let mut res = baseline.clone();
    //    for i in 0..N {
    //        res = apply_ops(res.clone(), cards.clone());
    //    }
    //
    //    assert_eq!(modpowed_poly.get_val(pos) as u32, res[pos as usize]);
    dbg!(modpowed_poly.get_val(pos));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let inp = include_str!("day_22_data.txt");
        let cards = get_ops(inp);
        let baseline = (0..=10006).collect::<Vector<u32>>();
        let res = apply_ops(baseline, cards);
        assert_eq!(res.len(), 10007);

        assert_eq!(res.index_of(&2019).unwrap(), 6417);
    }
    #[test]
    fn deal_new() {
        let inp: Vector<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let res = deal_into_new(deal_into_new(inp.clone()));
        assert_eq!(res, inp)
    }
    #[test]
    fn deal_inc() {
        let inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let res = deal_with_increment(inp, 3);
        assert_eq!(
            res,
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3].into_iter().collect()
        )
    }
    #[test]
    fn deal_inc9() {
        let inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            .into_iter()
            .collect::<Vector<u32>>();
        let res = deal_with_increment(inp.clone(), 10 * 1000 + 1);
        assert_eq!(res, inp)
    }
    #[test]
    fn deal_inc7() {
        let inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();

        let res = deal_with_increment(inp, 7);
        assert_eq!(
            res,
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7].into_iter().collect()
        )
    }
    #[test]
    fn deal_stack() {
        let inp: Vector<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let res = deal_into_new(inp.clone());
        assert_eq!(res, inp.into_iter().rev().collect())
    }
    #[test]
    fn deal_cut() {
        let inp: Vector<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let res = cut_n_cards(inp.clone(), 3);
        assert_eq!(
            res,
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2].into_iter().collect()
        )
    }
    #[test]
    fn deal_cut_neg() {
        let inp: Vector<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
        let res = cut_n_cards(inp.clone(), -4);
        assert_eq!(
            res,
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5].into_iter().collect()
        )
    }
    #[test]
    fn test_p1() {
        let inp = "deal with increment 7
deal into new stack
deal into new stack";
        let cards = get_ops(inp);
        let baseline = (0..10).collect::<Vector<u32>>();
        let res = apply_ops(baseline, cards);
        assert_eq!(
            res,
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
                .into_iter()
                .collect::<Vector<u32>>()
        )
    }
    #[test]
    fn test_rev() {
        let testv = vec![0; 300].into_iter().collect::<Vector<u32>>();
        let ret = deal_into_new(testv.clone());
        assert_eq!(ret.len(), testv.len());
    }
    #[test]
    fn test_p2() {
        let inp = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let cards = get_ops(inp);
        let baseline = (0..10).collect::<Vector<u32>>();
        let res = apply_ops(baseline, cards);
        assert_eq!(
            res,
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
                .into_iter()
                .collect::<Vector<u32>>()
        )
    }
}
