use std::{cmp::Ordering, collections::HashSet};

use regex::Regex;

type Vector3 = [i64; 3];

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Moon> {
    input.lines()
        .map(|l| l.into())
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Moon]) -> i64 {
    let mut input = input.to_vec();

    for _ in 0..1000 {
        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j {
                    continue;
                }
                let p = input[j].position;
                input[i].apply_p(&p);
            }
        }
        
        for e in &mut input {
            e.apply_v()
        }
    }

    input.iter()
        .map(|m| m.energy())
        .sum::<i64>()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Moon]) -> usize {
    let mut input = input.to_vec();

    let (mut past_x, mut past_y, mut past_z) = (HashSet::new(), HashSet::new(), HashSet::new());
    let (mut turn_x, mut turn_y, mut turn_z) = (None, None, None);

    for turn in 0_usize.. {
        let state_x: Vec<_> = input.iter().map(|m| (m.position[0], m.velocity[0])).collect();
        let state_y: Vec<_>  = input.iter().map(|m| (m.position[1], m.velocity[1])).collect();
        let state_z: Vec<_>  = input.iter().map(|m| (m.position[2], m.velocity[2])).collect();
        
        if !past_x.insert(state_x) && turn_x.is_none() {
            turn_x = Some(turn);
        }

        if !past_y.insert(state_y) && turn_y.is_none() {
            turn_y = Some(turn);
        }

        if !past_z.insert(state_z) && turn_z.is_none() {
            turn_z = Some(turn);
        }

        if turn_x.is_some() && turn_y.is_some() && turn_z.is_some() {
            break;
        }

        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j {
                    continue;
                }
                let p = input[j].position;
                input[i].apply_p(&p);
            }
        }

        for m in &mut input {
            m.apply_v();
        }
    }

    let (turn_x, turn_y, turn_z) = (turn_x.unwrap(), turn_y.unwrap(), turn_z.unwrap());
    lcm(lcm(turn_x, turn_y), turn_z)
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[derive(Clone)]
pub struct Moon {
    position: Vector3,
    velocity: Vector3
}

impl Moon {
    fn new(position: Vector3) -> Self {
        Self {
            position,
            velocity: [0; 3]
        }
    }

    fn apply_p(&mut self, other: &Vector3) {
        for (i, e) in self.velocity.iter_mut().enumerate() {
            let diff = other[i] - self.position[i];
            *e += if diff == 0 {
                0
            } else {
                diff / diff.abs()
            };
        }
    }

    fn apply_v(&mut self) {
        for (i, e) in self.position.iter_mut().enumerate() {
            *e += self.velocity[i];
        }
    }

    fn energy(&self) -> i64 {
        self.position.iter().map(|x| i64::abs(*x)).sum::<i64>() * 
        self.velocity.iter().map(|x| i64::abs(*x)).sum::<i64>()
    }
}

impl From<&str> for Moon {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
    
        let caps = re.captures(s).unwrap();
        let position: Vector3 = [
            caps["x"].parse().unwrap(),
            caps["y"].parse().unwrap(),
            caps["z"].parse().unwrap()
        ];

        Self::new(position)
    }
}

#[test]
fn test1() {
    let s = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    assert_eq!(179, part1(&generator(s)));
}