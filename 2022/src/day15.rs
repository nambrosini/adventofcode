use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;

type Position = (i64, i64);

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"Sensor at x=(?P<x1>-?\d+), y=(?P<y1>-?\d+): closest beacon is at x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap();

    input.lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let p1 = (caps["x1"].parse().unwrap(), caps["y1"].parse().unwrap());
            let p2 = (caps["x2"].parse().unwrap(), caps["y2"].parse().unwrap());

            Sensor { sensor: p1, beacon: p2 }
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(map: &[Sensor]) -> usize {
    cannot_exist( map, 2000000)
}

#[aoc(day15, part2)]
pub fn part2(map: &[Sensor]) -> i64 {
    find_beacon(map, 4000000)
}

fn is_in_area(sensor: &Sensor, upper: i64, target: i64) -> (bool, i64, i64) {
    let mh = sensor.distance(&sensor.beacon);
    let x_min = (sensor.sensor.0 - mh).max(0);
    let x_max = (sensor.sensor.0 + mh).min(upper);
    if x_min > upper || x_max < 0 {
        return (false, 0, 0)
    }
    let dist = mh - sensor.distance(&(sensor.sensor.0, target));
    if dist < 0 {
        return (false, 0, 0);
    }
    (true, (sensor.sensor.0 - dist).max(0), (sensor.sensor.0 + dist).min(upper))
}

pub struct Sensor {
    sensor: Position,
    beacon: Position
}

impl Sensor {
    fn distance(&self, other: &Position) -> i64 {
        (self.sensor.0 - other.0).abs() + (self.sensor.1 - other.1).abs()
    }
}

fn cannot_exist(sensors: &[Sensor], row: i64) -> usize {
    let x_min = sensors.iter()
        .map(|s| s.sensor.0 - s.distance(&s.beacon))
        .min().unwrap();
    let x_max = sensors.iter()
        .map(|s| s.sensor.0 + s.distance(&s.beacon))
        .max()
        .unwrap();

    let mut set = HashSet::new();

    for x in x_min..=x_max {
        if sensors.iter().any(|s| s.distance(&(x, row)) <= s.distance(&s.beacon)) {
            set.insert(x);
        }
    }

    for beacon in sensors.iter().map(|s| s.beacon).filter(|b| b.1 == row) {
        if set.contains(&beacon.0) {
            set.remove(&beacon.0);
        }
    }

    set.len()
}

fn find_beacon(sensors: &[Sensor], max: i64) -> i64 {
    for y in 0..=max {
        let mut x = 0;
        while x < max {
            let mut ranges = vec![];
            for sensor in sensors {
                let (isin, x_min, x_max) = is_in_area(sensor, max, y);
                if isin {
                    ranges.push((x_min, x_max));
                }
            }
            for (x_min, x_max) in ranges.iter().sorted() {
                if x_min <= &x && &x <= x_max {
                    x = *x_max;
                }
                if x >= max {
                    break
                }
            }
            if x != max {
                return (x + 1) * 4000000 + y;
            }
        }
    }

    unreachable!()
}

#[test]
fn test() {
    let s = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(26, cannot_exist(&generator(s), 10))
}

#[test]
fn test2() {
    let s = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(56000011, find_beacon( &generator(s), 20))
}