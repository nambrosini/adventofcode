use std::convert::{TryFrom, TryInto};
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug)]
pub enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32)
}

#[derive(Debug, Clone)]
pub enum Orientation {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
pub struct Ship {
    position: (i32, i32),
    orientation: Orientation
}

#[derive(Debug)]
pub struct WayPoint {
    position: (i32, i32),
    orientation: Orientation
}

impl Add<i32> for Orientation {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        match rhs {
            0 => self.clone(),
            90 => match self {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North
            },
            180 => match self {
                Orientation::North => Orientation::South,
                Orientation::East => Orientation::West,
                Orientation::South => Orientation::North,
                Orientation::West => Orientation::East
            },
            270 => match self {
                Orientation::North => Orientation::West,
                Orientation::East => Orientation::North,
                Orientation::South => Orientation::East,
                Orientation::West => Orientation::South
            },
            _ => unimplemented!()
        }
    }
}

impl AddAssign<i32> for Orientation {
    fn add_assign(&mut self, rhs: i32) {
        *self = self.clone() + rhs
    }
}

impl Sub for Orientation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let val1: i8 = match self{
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3
        };

        let val2: i8 = match rhs {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3
        };

        match (val2 - val1).abs() {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unimplemented!()
        }
    }
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let key = &v[0..1];
        let val: i32 = v[1..].parse().unwrap();

        Ok(match key {
            "N" => Action::N(val),
            "S" => Action::S(val),
            "E" => Action::E(val),
            "W" => Action::W(val),
            "L" => Action::L(val),
            "R" => Action::R(val),
            "F" => Action::F(val),
            _ => unreachable!()
        })
    }
}

impl Ship {
    fn new() -> Self {
        Self {
            position: (0, 0),
            orientation: Orientation::East
        }
    }

    fn run_part1(&mut self, actions: &[Action]) -> i32 {
        for action in actions {
            match action {
                Action::N(v) => self.position.1 += v,
                Action::S(v) => self.position.1 -= v,
                Action::E(v) => self.position.0 += v,
                Action::W(v) => self.position.0 -= v,
                Action::L(v) => self.orientation += 360 - *v,
                Action::R(v) => self.orientation += *v,
                Action::F(v) => self.advance(v)
            }
        }

        self.manhattan()
    }

    fn run_part2(&mut self, actions: &[Action]) -> i32 {
        let mut waypoint = WayPoint::new();

        for action in actions {
            match action {
                Action::N(v) => waypoint.position.1 += v,
                Action::S(v) => waypoint.position.1 -= v,
                Action::E(v) => waypoint.position.0 += v,
                Action::W(v) => waypoint.position.0 -= v,
                Action::L(v) => waypoint.rotate(waypoint.orientation.clone() + (360 - *v)),
                Action::R(v) => waypoint.rotate(waypoint.orientation.clone() + *v),
                Action::F(v) => self.move_to_waypoint(v, &waypoint)
            }
        }

        self.manhattan()
    }

    fn advance(&mut self, v: &i32) {
        match self.orientation {
            Orientation::North => self.position.1 += v,
            Orientation::East => self.position.0 += v,
            Orientation::South => self.position.1 -= v,
            Orientation::West => self.position.0 -= v
        }
    }

    fn move_to_waypoint(&mut self, v: &i32, waypoint: &WayPoint) {
        self.position.0 += waypoint.position.0 * v;
        self.position.1 += waypoint.position.1 * v;
    }

    fn manhattan(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }
}

impl WayPoint {
    fn new() -> Self{
        Self {
            position: (10, 1),
            orientation: Orientation::East
        }
    }

    fn rotate(&mut self, new_orientation: Orientation) {
        match self.orientation {
            Orientation::North => {
                match new_orientation {
                    Orientation::North => {},
                    Orientation::South => self.rotate_n(2),
                    Orientation::East => self.rotate_n(1),
                    Orientation::West => self.rotate_n(3)
                }
            },
            Orientation::South => {
                match new_orientation {
                    Orientation::North => self.rotate_n(2),
                    Orientation::South => {},
                    Orientation::East => self.rotate_n(3),
                    Orientation::West => self.rotate_n(1)
                }
            },
            Orientation::East => {
                match new_orientation {
                    Orientation::North => self.rotate_n(3),
                    Orientation::South => self.rotate_n(1),
                    Orientation::East => {},
                    Orientation::West => self.rotate_n(2)
                }
            },
            Orientation::West => {
                match new_orientation {
                    Orientation::North => self.rotate_n(1),
                    Orientation::South => self.rotate_n(3),
                    Orientation::East => self.rotate_n(2),
                    Orientation::West => {}
                }
            }
        }

        self.orientation = new_orientation;
    }

    fn rotate_n(&mut self, x: i32) {
        for _ in 0..x {
            self.position = (self.position.1, -self.position.0);
        }
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Action> {
    input.lines()
        .map(|l| l.try_into().unwrap())
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Action]) -> i32 {
    Ship::new().run_part1(input)
}

#[aoc(day12, part2)]
pub fn part2(input: &[Action]) -> i32 {
    Ship::new().run_part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let data = generator(&std::fs::read_to_string("tests/day12/sample").unwrap());

        assert_eq!(part1(&data), 25);
    }

    #[test]
    fn sample_part2() {
        let data = generator(&std::fs::read_to_string("tests/day12/sample").unwrap());

        assert_eq!(part2(&data), 286);
    }
}