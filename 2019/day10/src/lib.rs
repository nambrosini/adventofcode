use std::collections::HashSet;
use std::str::FromStr;
use std::f64::consts::PI;

pub fn solve_part_1(input: &str) -> usize {
    let asteroids = parse(input);
    Map::new(&asteroids).max
}

pub fn solve_part_2(input: &str) -> usize {
    let asteroids = parse(input);

    let map: Map = Map::new(&asteroids);

    let kill_order = map.get_kill_order();

    kill_order[199].x * 100 + kill_order[199].y
}

fn parse(input: &str) -> Vec<Point> {
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

    let asteroids: Vec<Point> = asteroids
        .iter()
        .flatten()
        .filter(|a| a.point_type == PointType::Asteroid)
        .map(|a| a.clone())
        .collect();

    asteroids
}

#[derive(Debug, PartialEq, Clone)]
pub enum PointType {
    Asteroid,
    Empty,
}
#[derive(PartialEq, Debug, Clone)]
struct Point {
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
            .map(|a| a.asteroids_seen(&input))
            .max()
            .unwrap();
        let base_index = input
            .iter()
            .enumerate()
            .find(|(_, a)| a.asteroids_seen(&input) == max)
            .unwrap()
            .0;
        let mut other: Vec<Point> = input.to_vec().clone();
        let center = other.remove(base_index);

        println!("center: {:?}", center);

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
                let min = angles.iter().min().unwrap().clone();
                let (index, _) = v
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x.angle.unwrap() == min)
                    .min_by_key(|(_, x)| x.distance.unwrap())
                    .unwrap()
                    .clone();
                kill_order.push(v.remove(index));
                angles.remove(&min);
            }
        }

        kill_order
    }
}

impl FromStr for PointType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(PointType::Empty),
            "#" => Ok(PointType::Asteroid),
            _ => Err(format!("Unknown type: {}", s)),
        }
    }
}

impl Point {
    fn new(x: usize, y: usize, s: &str) -> Self {
        Self {
            x,
            y,
            point_type: s.parse().unwrap(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = ".#..#
.....
#####
....#
...##";

        assert_eq!(solve_part_1(&input), 8);
    }

    #[test]
    fn test_two() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        assert_eq!(solve_part_1(&input), 33);
    }

    #[test]
    fn test_three() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        assert_eq!(solve_part_1(&input), 35);
    }

    #[test]
    fn test_four() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        assert_eq!(solve_part_1(&input), 41);
    }

    #[test]
    fn test_five() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(solve_part_1(&input), 210);
    }

    #[test]
    fn test_part_2() {
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        assert_eq!(solve_part_2(&input), 802);
    }

    #[test]
    fn test_asdf() {
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

        assert_eq!(solve_part_2(&input), 0);
    }
}
