use itertools::Itertools;
use std::convert::{From, Into};
use std::ops::Add;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Direction> {
    input.split(',').map(|x| x.into()).collect_vec()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Direction]) -> i32 {
    let mut position = Position::new(0, 0);

    for s in input {
        position = position + s.into()
    }

    Position::new(0, 0).calc_distance(position)
}

#[aoc(day11, part2)]
pub fn part2(input: &[Direction]) -> i32 {
    let start_position = Position::new(0, 0);
    let mut position = start_position;
    let mut max_distance = 0;

    for s in input {
        position = position + s.into();

        let dist = start_position.calc_distance(position);

        if dist > max_distance {
            max_distance = dist;
        }
    }

    max_distance
}

pub enum Direction {
    N,
    Ne,
    Se,
    S,
    Sw,
    Nw,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Direction {
        match value {
            "n" => Direction::N,
            "ne" => Direction::Ne,
            "se" => Direction::Se,
            "s" => Direction::S,
            "sw" => Direction::Sw,
            "nw" => Direction::Nw,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    q: i32,
    r: i32,
}

impl Position {
    fn new(q: i32, r: i32) -> Position {
        Self { q, r }
    }

    fn calc_distance(&self, other: Position) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r))
            / 2
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::N => Position::new(0, -1),
            Direction::Ne => Position::new(1, -1),
            Direction::Se => Position::new(1, 0),
            Direction::S => Position::new(0, 1),
            Direction::Sw => Position::new(-1, 1),
            Direction::Nw => Position::new(-1, 0),
        }
    }
}
