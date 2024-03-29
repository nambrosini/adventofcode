use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Vec<Node> {
    let re = Regex::new(r"x(\d+)-y(\d+).+?(\d+).+?(\d+).+?(\d+).+?(\d+)").unwrap();
    input
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|cap| Node {
                coords: Coords::new(
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                used: cap.get(4).unwrap().as_str().parse().unwrap(),
                available: cap.get(5).unwrap().as_str().parse().unwrap(),
            })
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &[Node]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| are_compatible(a, b) || are_compatible(b, a))
        .count()
}

#[aoc(day22, part2)]
pub fn part2(nodes: &[Node]) -> u32 {
    let max = nodes.iter().map(|n| n.coords).max().unwrap();
    let goal = nodes
        .iter()
        .find(|n| n.coords.x == max.x && n.coords.y == 0)
        .unwrap();

    let min_used = nodes
        .iter()
        .filter_map(|n| if n.used > 0 { Some(n.used) } else { None })
        .min()
        .unwrap();
    let mut empty_nodes = nodes.iter().filter(|n| n.available >= min_used);
    let empty = empty_nodes.next().unwrap().clone();
    assert_eq!(empty_nodes.next(), None);

    let empty_size = empty.available;

    let params = Params {
        max,
        full: nodes
            .iter()
            .filter(|n| n.used > empty_size)
            .map(|n| n.coords)
            .collect(),
    };

    find_path_to(
        &params,
        empty.coords,
        Coords::new(goal.coords.x - 1, goal.coords.y),
    )
    .unwrap()
        + (goal.coords.x as u32 - 1) * 5
        + 1
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
struct Coords {
    x: i16,
    y: i16,
}

impl Coords {
    fn new(x: i16, y: i16) -> Self {
        Coords { x, y }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Node {
    coords: Coords,
    used: u16,
    available: u16,
}

#[derive(Debug)]
struct Params {
    max: Coords,
    full: BTreeSet<Coords>,
}

impl Params {
    fn next(&self, coords: Coords) -> Vec<Coords> {
        [
            (coords.x + 1, coords.y),
            (coords.x - 1, coords.y),
            (coords.x, coords.y + 1),
            (coords.x, coords.y - 1),
        ]
        .iter()
        .map(|&(x, y)| Coords::new(x, y))
        .filter(|coord| {
            coord.x >= 0 && coord.y >= 0 && coord.x <= self.max.x && coord.y <= self.max.y
        })
        .filter(|coord| !self.full.contains(coord))
        .collect()
    }
}

fn are_compatible(a: &Node, b: &Node) -> bool {
    a.used > 0 && a.used <= b.available
}

fn find_path_to(params: &Params, position: Coords, goal: Coords) -> Option<u32> {
    let mut map = BTreeMap::new();
    map.insert(position, 0);

    for i in 0.. {
        let aa: Vec<_> = map
            .iter()
            .filter(|&(_, v)| v == &i)
            .map(|(v, _)| *v)
            .collect();
        let mut should_go_next = false;
        for p in &aa {
            for next in &params.next(*p) {
                if *next == goal {
                    return Some(i + 1);
                }
                if !map.contains_key(next) {
                    should_go_next = true;
                    map.insert(*next, i + 1);
                }
            }
        }

        if !should_go_next {
            return None;
        }
    }
    unreachable!();
}
