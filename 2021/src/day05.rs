use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day05)]
pub fn generator(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[aoc(day05, part1)]
pub fn part1(input: &[Line]) -> usize {
    let mut map = HashMap::new();

    for line in input {
        if line.is_hv_line() {
            let v = line.get_points();

            for p in v {
                let entry = map.entry(p).or_insert(0);
                *entry += 1;
            }
        }
    }

    map.iter().filter(|(_, &v)| v > 1).count()
}

#[aoc(day05, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut map = HashMap::new();

    for line in input {
        let v = line.get_points();

        for p in v {
            let entry = map.entry(p).or_insert(0);
            *entry += 1;
        }
    }

    map.iter().filter(|(_, &v)| v > 1).count()
}

pub struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

        let caps = re.captures(s).unwrap();

        let p1 = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let p2 = (caps[3].parse().unwrap(), caps[4].parse().unwrap());

        if p1.0 + p1.1 < p2.0 + p2.1 {
            Self { from: p1, to: p2 }
        } else {
            Self { from: p2, to: p1 }
        }
    }
}

impl Line {
    fn is_hv_line(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }

    fn get_points(&self) -> Vec<(i32, i32)> {
        let mut v = vec![];

        if self.from.0 == self.to.0 {
            let dist = (self.to.1 - self.from.1).abs();
            for i in 0..=dist {
                v.push((self.from.0, self.from.1 + i));
            }
        } else if self.from.1 == self.to.1 {
            let dist = (self.to.0 - self.from.0).abs();
            for i in 0..=dist {
                v.push((self.from.0 + i, self.from.1));
            }
        } else {
            let dist = (self.from.0 - self.to.0).abs();

            for i in 0..=dist {
                let x = if self.from.0 > self.to.0 {
                    self.from.0 - i
                } else {
                    self.from.0 + i
                };
                let y = if self.from.1 > self.to.1 {
                    self.from.1 - i
                } else {
                    self.from.1 + i
                };
                v.push((x, y));
            }
        }

        v
    }
}
#[test]
fn test() {
    let s = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    let s = generator(s);

    assert_eq!(part1(&s), 5);
}

#[test]
fn test2() {
    let s = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    let s = generator(s);

    assert_eq!(part2(&s), 12);
}
