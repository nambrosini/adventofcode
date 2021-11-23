extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use regex::Regex;

pub fn solve(input: &str) {
    {
        let mut moons: Vec<Moon> = input.lines().map(|l| l.parse().unwrap()).collect();

        for _ in 0..1000 {
            for i in 0..moons.len() {
                for j in 0..moons.len() {
                    if i == j {
                        continue;
                    }
                    let p = moons[j].position;
                    moons[i].apply_g(p);
                }
            }

            for m in &mut moons {
                m.apply_v();
            }
        }
    
        let e: i32 = moons.iter().map(Moon::energy).sum();
        println!("Part 1: {}", e);
    }
    
    // part 2
    {
        let mut moons: Vec<Moon> = input.lines().map(|l| l.parse().unwrap()).collect();
        let (mut past_x, mut past_y, mut past_z) = (HashSet::new(), HashSet::new(), HashSet::new());
        let (mut turn_x, mut turn_y, mut turn_z) = (None, None, None);

        for turn in 0_usize.. {
            let state_x: Vec<_> = moons.iter().map(|m| (m.position.x, m.velocity.x)).collect();
            let state_y: Vec<_> = moons.iter().map(|m| (m.position.y, m.velocity.y)).collect();
            let state_z: Vec<_> = moons.iter().map(|m| (m.position.z, m.velocity.z)).collect();

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

            for i in 0..moons.len() {
                for j in 0..moons.len() {
                    if i == j {
                        continue;
                    }
                    let p = moons[j].position;
                    moons[i].apply_g(p);
                }
            }

            for m in &mut moons {
                m.apply_v();
            }
        }

        let (turn_x, turn_y, turn_z) = (turn_x.unwrap(), turn_y.unwrap(), turn_z.unwrap());
        let common = lcm(lcm(turn_x, turn_y), turn_z);
        println!("Part 2: {}", common);
    }
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

#[derive(PartialEq, Clone, Debug, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point { x, y, z }
    }

    fn calc(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"<x=(?P<x>(-\d+|\d+)), y=(?P<y>(-\d+|\d+)), z=(?P<z>(-\d+|\d+))>")
                    .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        Ok(Point::new(
            caps["x"].parse()?,
            caps["y"].parse()?,
            caps["z"].parse()?,
        ))
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Copy)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl FromStr for Moon {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Moon {
            position: s.parse()?,
            velocity: Point::new(0, 0, 0),
        })
    }
}

impl Moon {
    fn apply_g(&mut self, o: Point) {
        let x = o.x - self.position.x;
        let y = o.y - self.position.y;
        let z = o.z - self.position.z;

        self.velocity.x += if x == 0 { 0 } else { x / x.abs() };
        self.velocity.y += if y == 0 { 0 } else { y / y.abs() };
        self.velocity.z += if z == 0 { 0 } else { z / z.abs() };
    }
    
    fn apply_v(&mut self) {
        self.position += self.velocity.clone();
    }

    fn energy(&self) -> i32 {
        self.position.calc() * self.velocity.calc()
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = std::fs::read_to_string("test").unwrap();

        solve(&input);
    }
}
