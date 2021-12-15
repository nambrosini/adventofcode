use std::collections::HashSet;
use std::f64::consts::PI;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Point> {
    let asteroids: Vec<Vec<Point>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Point::new(j, i, &c.to_string()))
                .collect()
        })
        .collect();

    asteroids
        .iter()
        .flatten()
        .filter(|a| a.point_type == PointType::Asteroid)
        .cloned()
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(asteroids: &[Point]) -> usize {
    Map::new(asteroids).max
}

#[aoc(day10, part2)]
pub fn part2(asteroids: &[Point]) -> usize {
    let map: Map = Map::new(asteroids);

    let kill_order = map.get_kill_order();

    kill_order[199].x * 100 + kill_order[199].y
}

#[derive(Debug, PartialEq, Clone)]
pub enum PointType {
    Asteroid,
    Empty,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
    point_type: PointType,
    angle: Option<i32>,
    distance: Option<i32>,
}

struct Map {
    other: Vec<Point>,
    max: usize,
}

impl Map {
    fn new(input: &[Point]) -> Self {
        let max = input
            .iter()
            .map(|a| a.asteroids_seen(input))
            .max()
            .unwrap();
        let base_index = input
            .iter()
            .enumerate()
            .find(|(_, a)| a.asteroids_seen(input) == max)
            .unwrap()
            .0;
        let mut other: Vec<Point> = input.to_vec();
        let center = other.remove(base_index);

        for i in &mut other {
            i.calc_angle_and_distance_from(&center);
        }

        Self { other, max }
    }

    fn get_kill_order(&self) -> Vec<Point> {
        let mut v: Vec<Point> = self.other.clone();
        let mut kill_order: Vec<Point> = vec![];

        loop {
            let mut angles: HashSet<i32> = v.iter().map(|e| e.angle.unwrap()).collect();

            if angles.is_empty() {
                break;
            }

            while !angles.is_empty() {
                let min = *angles.iter().min().unwrap();
                let (index, _) = v
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x.angle.unwrap() == min)
                    .min_by_key(|(_, x)| x.distance.unwrap())
                    .unwrap();
                kill_order.push(v.remove(index));
                angles.remove(&min);
            }
        }

        kill_order
    }
}

impl From<&str> for PointType {
    fn from(s: &str) -> Self {
        match s {
            "." => PointType::Empty,
            "#" => PointType::Asteroid,
            _ => unreachable!()
        }
    }
}

impl Point {
    fn new(x: usize, y: usize, s: &str) -> Self {
        Self {
            x,
            y,
            point_type: s.into(),
            angle: None,
            distance: None,
        }
    }

    fn get_step(&self, other: &Point) -> (i32, i32) {
        let (diff_x, diff_y) = (
            other.x as i32 - self.x as i32,
            other.y as i32 - self.y as i32,
        );
        let mcd = mcd(diff_x, diff_y);
        (diff_x / mcd, diff_y / mcd)
    }

    fn asteroids_seen(&self, map: &[Point]) -> usize {
        let set: HashSet<(i32, i32)> = map
            .iter()
            .filter(|&a| a != self)
            .map(|a| self.get_step(a))
            .collect();

        set.len()
    }

    fn calc_angle_and_distance_from(&mut self, center: &Point) {
        let dist_x: f64 = self.x as f64 - center.x as f64;
        let dist_y: f64 = self.y as f64 - center.y as f64;
        let angle = dist_y.atan2(dist_x) + PI / 2.0;
        let angle = if angle < 0.0 {
            angle + 2.0 * PI
        } else {
            angle
        };

        self.angle = Some((angle * 1_000_000.0) as i32);
        self.distance = Some(((dist_x.powi(2) + dist_y.powi(2)).sqrt() * 1_000_000.0) as i32);
    }
}

fn mcd(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();

    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}