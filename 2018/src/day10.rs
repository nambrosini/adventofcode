use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use regex::Regex;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Map {
    let points: Vec<Point> = input.lines().map(|l| l.into()).collect();

    Map::new(points)
}

#[aoc(day10, part1)]
pub fn part1(map: &Map) -> usize {
    let mut map: Map = map.clone();
    for _ in 1..1_000_000 {
        map.simulate();

        if map.get_height() <= 100 && map.get_width() <= 100 {
            println!("{}", map);
        }
    }
    0
}

#[aoc(day10, part2)]
pub fn part2(map: &Map) -> usize {
    let mut map: Map = map.clone();
    for i in 1..1_000_000 {
        map.simulate();

        if map.get_height() <= 80 && map.get_width() <= 80 {
            println!("{}", i);
            println!("{}", map);
        }
    }
    0
}

#[derive(Clone)]
pub struct Map {
    points: Vec<Point>,
}

impl Map {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn simulate(&mut self) {
        self.points.iter_mut().for_each(|p| p.tick())
    }

    fn get_width(&self) -> i64 {
        let min_x = self
            .points
            .iter()
            .min_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let max_x = self
            .points
            .iter()
            .max_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;

        max_x - min_x
    }

    fn get_height(&self) -> i64 {
        let min_y = self
            .points
            .iter()
            .min_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;
        let max_y = self
            .points
            .iter()
            .max_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;

        max_y - min_y
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self
            .points
            .iter()
            .min_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let max_x = self
            .points
            .iter()
            .max_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let min_y = self
            .points
            .iter()
            .min_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;
        let max_y = self
            .points
            .iter()
            .max_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let vec = Vec2::new(x, y);
                if self.points.iter().any(|p| p.position == vec) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    position: Vec2<i64>,
    velocity: Vec2<i64>,
}

impl Point {
    fn tick(&mut self) {
        self.position += self.velocity;
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"position=<\s*(?P<p_x>-?\d+),\s*(?P<p_y>-?\d+)> velocity=<\s*(?P<v_x>-?\d+),\s*(?P<v_y>-?\d+)>").unwrap();

        let caps = re.captures(s).unwrap();

        let position: Vec2<i64> =
            Vec2::new(caps["p_x"].parse().unwrap(), caps["p_y"].parse().unwrap());
        let velocity: Vec2<i64> =
            Vec2::new(caps["v_x"].parse().unwrap(), caps["v_y"].parse().unwrap());

        Self { position, velocity }
    }
}
