use std::collections::{HashMap, HashSet};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> (Wire, Wire) {
    let mut wires = input.lines().map(|line| line.into());

    (wires.next().unwrap(), wires.next().unwrap())
}

#[aoc(day3, part1)]
pub fn part1((wire0, wire1): &(Wire, Wire)) -> i64 {
    wire0.manhattan(wire1)
}

#[aoc(day3, part2)]
pub fn part2((wire0, wire1): &(Wire, Wire)) -> i64 {
    wire0.steps(wire1)
}

enum Dir {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Dir {
    fn from(input: &str) -> Self {
        let dir = &input[..1];
        let len = input[1..].parse().unwrap();

        match dir {
            "U" => Dir::Up(len),
            "D" => Dir::Down(len),
            "L" => Dir::Left(len),
            "R" => Dir::Right(len),
            _ => panic!("Direction unknown: {}", dir),
        }
    }
}

pub struct Wire {
    path: Vec<(i64, i64)>,
}

impl From<&str> for Wire {
    fn from(path: &str) -> Self {
        let path: Vec<Dir> = path.split(',').map(|x| x.into()).collect();

        let mut res: Vec<(i64, i64)> = vec![(0, 0)];

        for p in path {
            let (len, (dx, dy)) = match p {
                Dir::Up(v) => (v, (0, 1)),
                Dir::Down(v) => (v, (0, -1)),
                Dir::Left(v) => (v, (-1, 0)),
                Dir::Right(v) => (v, (1, 0)),
            };

            let (x, y) = *res.last().unwrap();

            res.extend((1..=(len as i64)).map(|d| (x + dx * d, y + dy * d)));
        }

        Wire { path: res }
    }
}

impl Wire {
    pub fn manhattan(&self, other: &Wire) -> i64 {
        let path1: HashSet<(i64, i64)> = self.path.clone().into_iter().skip(1).collect();
        let path2: HashSet<(i64, i64)> = other.path.clone().into_iter().skip(1).collect();
        let common_points: Vec<&(i64, i64)> = path1
            .iter()
            .skip(1)
            .filter(|e| path2.contains(&e))
            .collect();

        common_points
            .iter()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    }

    pub fn steps(&self, other: &Wire) -> i64 {
        let path1: HashMap<(i64, i64), usize> = self
            .path
            .iter()
            .enumerate()
            .map(|(i, (x, y))| ((*x, *y), i))
            .rev()
            .collect();

        let common_points = other
            .path
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(d2, (x, y))| {
                let d1 = path1.get(&(*x, *y))?;
                Some(d1 + d2)
            });

        common_points.min().unwrap() as i64
    }
}