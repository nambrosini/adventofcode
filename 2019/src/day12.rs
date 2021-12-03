use regex::Regex;
use std::cmp::Ordering;
use std::fmt;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Moon> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc(day12, part1)]
pub fn part1(moons: &[Moon]) -> i32 {
    simulate(moons, 1000)
}

pub fn simulate(moons: &[Moon], steps: usize) -> i32 {
    let mut moons = moons.to_vec();

    for i in 1..=steps {
        let ms = moons.clone();
        for m in moons.iter_mut() {
            m.step(&ms);
        }
    }

    moons.iter()
        .map(|m| m.calc_energy())
        .sum()
}

#[derive(Debug, Copy, Clone)]
pub struct Moon {
    position: [i32; 3],
    velocity: [i32; 3]
}

impl Moon {
    fn calc_vel(&mut self, moons: &[Moon]) {
        let mut new_vel: [i32; 3] = self.velocity;

        for m in moons {
            for (i, e) in new_vel.iter_mut().enumerate() {
                *e += match self.position[i].cmp(&m.position[i]) {
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                    Ordering::Less => 1
                };
            }
        }

        self.velocity = new_vel;
    }

    fn calc_pos(&mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }

    fn step(&mut self, moons: &[Moon]) {
        self.calc_vel(moons);
        self.calc_pos();
    }

    fn calc_energy(&self) -> i32 {
        let pot = self.position.iter().fold(0, |sum, pos| sum + pos.abs());
        let kin = self.velocity.iter().fold(0, |sum, pos| sum + pos.abs());
        pot * kin
    }
}

impl From<&str> for Moon {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

        let caps = re.captures(s).unwrap();

        let position = [caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap()];
        let velocity = [0; 3];

        Self {
            position,
            velocity,
        }
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.position[0],
            self.position[1],
            self.position[2],
            self.velocity[0],
            self.velocity[1],
            self.velocity[2],
        );

        write!(f, "{}", s)
    }
}

#[test]
pub fn test11() {
    let s = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    let moons = generator(s);

    assert_eq!(simulate(&moons, 10), 179);
}

#[test]
pub fn test12() {
    let s = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    let moons = generator(s);

    assert_eq!(simulate(&moons, 100), 1940);
}