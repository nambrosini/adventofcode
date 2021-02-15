use itertools::Itertools;
use std::convert::{TryFrom, TryInto, From, Into};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    pub fn rotate(&self, step: &Step) -> Direction {
        match self {
            Direction::N => match step {
                Step::R(_) => Direction::E,
                Step::L(_) => Direction::W
            },
            Direction::E => match step {
                Step::R(_) => Direction::S,
                Step::L(_) => Direction::N
            },
            Direction::S => match step {
                Step::R(_) => Direction::W,
                Step::L(_) => Direction::E
            },
            Direction::W => match step {
                Step::R(_) => Direction::N,
                Step::L(_) => Direction::S
            }
        }
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => Position::new(0, 1),
            Direction::S => Position::new(0, -1),
            Direction::E => Position::new(1, 0),
            Direction::W => Position::new(-1, 0)
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Mul<u32> for Position {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Self::new(self.x * other as i32, self.y * other as i32)
    }
}

#[derive(Debug)]
pub enum Step {
    R(u32),
    L(u32)
}

impl TryFrom<&str> for Step {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dir = &value[..1];
        let val = &value[1..].parse::<u32>().unwrap();

        match dir {
            "R" => Ok(Step::R(*val)),
            "L" => Ok(Step::L(*val)),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct State {
    position: Position,
    direction: Direction
}

impl State {
    pub fn new() -> Self {
        Self {
            position: Position::new(0, 0),
            direction: Direction::N
        }
    }

    pub fn exec(&mut self, steps: &[Step]) {
        for s in steps {
            self.step(s);
        }
    }

    pub fn exec_part2(&mut self, steps: &[Step]) -> Position {
        let mut v = vec![self.position.clone()];
        for s in steps {
            for p in self.step(s) {
                if v.contains(&p) {
                    return p;
                }
                v.push(p);
            }
        }

        unreachable!();
    }

    pub fn step(&mut self, step: &Step) -> Vec<Position> {
        let new_dir = self.direction.rotate(step);
        let dir_pos: Position = new_dir.into();
        let val = match step {
            Step::R(val) => *val,
            Step::L(val) => *val
        };

        let mut positions = vec![];

        for _ in 0..val {
            self.position = self.position + dir_pos;
            positions.push(self.position.clone());
        }

        self.direction = new_dir;

        positions
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Step> {
    input.split(", ")
        .map(|step| step.try_into().unwrap())
        .collect_vec()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Step]) -> i32 {
    let mut state = State::new();
    state.exec(input);
    state.position.distance()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Step]) -> i32 {
    let mut state = State::new();
    let p = state.exec_part2(input);
    p.distance()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample1() {
        let s = input_generator("R2, L3");

        assert_eq!(part1(&s), 5);
    }

    #[test]
    fn sample2() {
        let s = input_generator("R2, R2, R2");

        assert_eq!(part1(&s), 2);
    }

    #[test]
    fn sample3() {
        let s = input_generator("R5, L5, R5, R3");

        assert_eq!(part1(&s), 12);
    }

    #[test]
    fn sample4() {
        let s = input_generator("R8, R4, R4, R8");

        assert_eq!(part2(&s), 4);
    }
}
