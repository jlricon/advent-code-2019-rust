//<x=14, y=4, z=5>
//<x=12, y=10, z=8>
//<x=1, y=7, z=-10>
//<x=16, y=-5, z=3>

use std::collections::HashMap;

use itertools::{zip_eq, Itertools};
use num::Integer;

type Velocity = [i32; 3];
type Position = [i32; 3];
type Axis = [i32; 6 * 4];
type Seens = Vec<HashMap<Axis, u64>>;
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Moon {
    velocity: Velocity,
    position: Position,
}
fn absum(x: [i32; 3]) -> i32 {
    x.iter().map(|x| x.abs()).sum()
}
fn apply_gravity(mut moons: [Moon; 4]) -> [Moon; 4] {
    // Applies gravity to two moons
    (0..=3)
        .tuple_combinations()
        .cartesian_product(0..=2)
        .for_each(|((a, b), i)| {
            // Runs through

            if moons[a].position[i] > moons[b].position[i] {
                moons[a].velocity[i] -= 1;
                moons[b].velocity[i] += 1;
            } else if moons[a].position[i] < moons[b].position[i] {
                moons[a].velocity[i] += 1;
                moons[b].velocity[i] -= 1;
            }
        });
    moons
}
fn apply_velocity(a: &mut Moon) {
    zip_eq(a.position.iter_mut(), a.velocity.iter()).for_each(|(pos, vel)| *pos += vel)
}

impl Moon {
    fn energy(&self) -> i32 {
        absum(self.position) * absum(self.velocity)
    }
    fn new(position: Position) -> Moon {
        Moon {
            velocity: [0, 0, 0],
            position,
        }
    }
}
fn part1() -> i32 {
    let mut moon_pos = [
        Moon::new([14, 4, 5]),
        Moon::new([12, 10, 8]),
        Moon::new([1, 7, -10]),
        Moon::new([16, -5, 3]),
    ];

    for _ in 1..=1000 {
        moon_pos = apply_gravity(moon_pos);
        moon_pos.iter_mut().for_each(|m| apply_velocity(m));
    }

    let e: i32 = moon_pos.iter().map(|m| m.energy()).sum();
    dbg!(e);
    e
}
fn part2() {
    let moon_pos = [
        Moon::new([14, 4, 5]),
        Moon::new([12, 10, 8]),
        Moon::new([1, 7, -10]),
        Moon::new([16, -5, 3]),
    ];
    let res = part2_for_vec(moon_pos);
    dbg!(res);
}
fn get_ax(pos: usize, moon_pos: &[Moon; 4]) -> Axis {
    let posv = moon_pos.iter().map(|m| m.position[pos]);
    let velv = moon_pos.iter().map(|m| m.velocity[pos]);
    let res = posv.chain(velv).collect_vec();
    let mut outp = [0; 24];
    res.into_iter()
        .enumerate()
        .for_each(|(pos, v)| outp[pos] = v);
    outp
}
fn foreach_ax(
    moon_pos: &[Moon; 4],
    i: u64,
    already_seen: &mut Seens,
    periods: &mut [Option<u64>; 3],
    fun: impl Fn(u64, usize, &mut Seens, Axis, &mut [Option<u64>; 3]),
) {
    (0..=2)
        .map(|p| get_ax(p, moon_pos))
        .enumerate()
        .for_each(|(pos, ax)| {
            fun(i, pos, already_seen, ax, periods);
        });
}
fn part2_for_vec(mut moon_pos: [Moon; 4]) -> u128 {
    // Vector with the OG values
    // Let's look at axis instead? x-6
    let mut periods: [Option<u64>; 3] = [None; 3];
    let mut i = 0_u64;

    let mut already_seen: Vec<HashMap<Axis, u64>> = vec![HashMap::new(); 3];

    foreach_ax(
        &moon_pos,
        i,
        &mut already_seen,
        &mut periods,
        |i, pos, seen, ax, _| {
            seen[pos].insert(ax, i);
        },
    );

    loop {
        moon_pos = step(moon_pos);
        i += 1;
        // For each moon, check if the state is in the hashset. If it is not, insert it.
        // If it is, add it to the relevant period
        foreach_ax(
            &moon_pos,
            i,
            &mut already_seen,
            &mut periods,
            |i, pos, seen, ax, periods| {
                if let Some(ord) = seen[pos].get(&ax) {
                    if periods[pos] == None {
                        periods[pos] = Some(i - ord)
                    }
                } else {
                    seen[pos].insert(ax, i);
                }
            },
        );

        if periods.iter().all(|f| *f != None) {
            break;
        }
    }
    let lcm: u128 = periods
        .iter()
        .filter_map(|v| match v {
            Option::None => Option::None,
            Some(e) => Some(*e as u128),
        })
        .fold(1, |acc, x| x.lcm(&acc));

    lcm as u128
}

fn step(mut moon_pos: [Moon; 4]) -> [Moon; 4] {
    moon_pos = apply_gravity(moon_pos);
    moon_pos.iter_mut().for_each(|m| apply_velocity(m));
    moon_pos
}

fn main() {
    part1();
    part2();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let p = part1();
        assert_eq!(p, 6423);
    }
    #[test]
    fn test_1() {
        let moon_pos = [
            Moon::new([0, 0, 0]),
            Moon::new([0, 0, 0]),
            Moon::new([0, 0, 0]),
            Moon::new([0, 0, 0]),
        ];
        let res = part2_for_vec(moon_pos);
        assert_eq!(res, 1);
    }
    #[test]
    fn test_2() {
        let moon_pos = [
            Moon::new([1, 0, 0]),
            Moon::new([-1, 0, 0]),
            Moon::new([0, 1, 0]),
            Moon::new([0, -1, 0]),
        ];
        let res = part2_for_vec(moon_pos);
        assert_eq!(res, 4);
        //(2,-2)/(0,0)
        //(-1,1)/(-3,3)
        //(-1,1)/(0,0)
        //(2,-2)/(3,-3)
        //(2,-2)/(0,0)
        let moon_pos = [
            Moon::new([0, 0, 2]),
            Moon::new([0, 0, 0]),
            Moon::new([0, 0, -2]),
            Moon::new([0, 0, 0]),
        ];
        let res = part2_for_vec(moon_pos);
        assert_eq!(res, 4);
        //(1,-1,2,-2)/(0,0,0,0)

        let moon_pos = [
            Moon::new([0, 0, 2]),
            Moon::new([0, 0, -2]),
            Moon::new([0, 0, 1]),
            Moon::new([0, 0, -1]),
        ];
        let exp = [
            Moon {
                position: [0, 0, -1],
                velocity: [0, 0, -3],
            },
            Moon {
                position: [0, 0, 1],
                velocity: [0, 0, 3],
            },
            Moon {
                position: [0, 0, 0],
                velocity: [0, 0, -1],
            },
            Moon {
                position: [0, 0, 0],
                velocity: [0, 0, 1],
            },
        ];
        let res = step(moon_pos);
        assert_eq!(res, exp);
    }
    #[test]
    fn test_real() {
        //        <x=-1, y=0, z=2>
        //            <x=2, y=-10, z=-7>
        //            <x=4, y=-8, z=8>
        //            <x=3, y=5, z=-1>
        let moon_pos = [
            Moon::new([-1, 0, 2]),
            Moon::new([2, -10, -7]),
            Moon::new([4, -8, 8]),
            Moon::new([3, 5, -1]),
        ];
        let res = part2_for_vec(moon_pos);
        assert_eq!(res, 2772);
    }
    #[test]
    fn test_real2() {
        //        <x=-8, y=-10, z=0>
        //            <x=5, y=5, z=10>
        //            <x=2, y=-7, z=3>
        //            <x=9, y=-8, z=-3>
        let moon_pos = [
            Moon::new([-8, -10, 0]),
            Moon::new([5, 5, 10]),
            Moon::new([2, -7, 3]),
            Moon::new([9, -8, -3]),
        ];
        let res = part2_for_vec(moon_pos);
        assert_eq!(res, 4686774924);
    }
    #[test]
    fn test_lcm() {
        let lcm: u128 = vec![2, 4, 6].into_iter().fold(1, |acc, x| (x.lcm(&acc)));
        assert_eq!(lcm, 12);
        let lcm: u128 = vec![2, 4, 6, 3, 47, 1]
            .into_iter()
            .fold(1, |acc, x| (x.lcm(&acc)));
        assert_eq!(lcm, 12 * 47);
    }
}
