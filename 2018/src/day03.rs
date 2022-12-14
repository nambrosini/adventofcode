use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::{From, Into};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Square> {
    input.lines().map(|l| l.into()).collect_vec()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Square]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    for s in input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map.entry((s.x + i, s.y + j)).or_insert(0);
                *e += 1;
            }
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Square]) -> usize {
    let mut map: HashMap<(usize, usize), String> = HashMap::new();

    let mut destroyed: HashSet<usize> = HashSet::new();

    for s in input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map
                    .entry((s.x + i, s.y + j))
                    .or_insert_with(|| ".".to_string());
                if e == "." {
                    *e = format!("{}", s.id);
                } else if e == "X" {
                    destroyed.insert(s.id);
                } else {
                    destroyed.insert(e.clone().parse().unwrap());
                    destroyed.insert(s.id);
                    *e = "X".to_string();
                }
            }
        }
    }

    input
        .iter()
        .map(|s| s.id)
        .find(|s| !destroyed.contains(s))
        .unwrap()
}

#[derive(Debug)]
pub struct Square {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Square {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        Self {
            id: cap[1].parse().unwrap(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            width: cap[4].parse().unwrap(),
            height: cap[5].parse().unwrap(),
        }
    }
}

#[test]
fn test1() {
    let s = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    let s = generator(s);

    assert_eq!(part1(&s), 4);
}

#[test]
fn test2() {
    let s = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    let s = generator(s);

    assert_eq!(part2(&s), 3);
}
