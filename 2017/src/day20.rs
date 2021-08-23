use std::ops::AddAssign;

use itertools::Itertools;
use regex::Regex;
    
#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<Particle> {
    input.lines().map(|x| x.into()).collect_vec()
}

#[aoc(day20, part1)]
pub fn part1(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();

    for _ in 0..10000 {
        particles.iter_mut().for_each(|p| p.step());
    }

    particles.iter().enumerate().map(|(i, p)| (i, p.manhattan())).min_by_key(|(_, e)| *e).unwrap().0
}

#[aoc(day20, part2)]
pub fn part2(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();

    for _ in 0..10000 {
        particles.iter_mut().for_each(|p| p.step());

        let mut no_collisions = vec![];

        for p in &particles {
            if particles.iter().filter(|&x| x == p).count() == 1 {
                no_collisions.push(*p);
            }
        }

        particles = no_collisions.clone();
    }

    particles.len()
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: i128,
    y: i128,
    z: i128
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<&str> for Vector {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"[pva]=<(-\d+|\d+),(-\d+|\d+),(-\d+|\d+)>").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        Self {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl From<&str> for Particle {
    fn from(s: &str) -> Self {
        let split = s.split(", ").collect_vec();

        Self {
            p: split[0].into(),
            v: split[1].into(),
            a: split[2].into()
        }
    }
}

impl Particle {
    fn step(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }

    fn manhattan(&self) -> i128 {
        self.p.x.abs() + self.p.y.abs() + self.p.z.abs()
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}