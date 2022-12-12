use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::usize;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    let mut input = input.to_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in input.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if e == &'S' {
                start = (i as i32, j as i32);
            }
            if e == &'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    input[start.0 as usize][start.1 as usize] = 'a';
    input[end.0 as usize][end.1 as usize] = 'z';

    astar(&input, &start, &end).len() - 1
}

#[aoc(day12, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let mut input = input.to_vec();
    let mut start = vec![];
    let mut end = (0, 0);

    for (i, row) in input.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if e == &'S' {
                start.insert(0, (i as i32, j as i32));
            } else if e == &'a' {
                start.push((i as i32, j as i32));
            } else if e == &'E' {
                end = (i as i32, j as i32);
            }
        }
    }

    input[start[0].0 as usize][start[0].1 as usize] = 'a';
    input[end.0 as usize][end.1 as usize] = 'z';

    start
        .iter()
        .map(|start| astar(&input, start, &end).len())
        .filter(|res| res != &0)
        .min()
        .unwrap()
        - 1
}

type Point = (i32, i32);

pub fn astar(map: &[Vec<char>], start: &Point, end: &Point) -> Vec<Point> {
    let mut open_list: HashSet<Node> = HashSet::new();
    let mut closed_list: HashSet<Node> = HashSet::new();

    let start = Node::new(*start, None);
    let end = Node::new(*end, None);

    open_list.insert(start);

    while !open_list.is_empty() {
        let current_node = *open_list.clone().iter().min_by_key(|n| n.f).unwrap();

        open_list.remove(&current_node);
        closed_list.insert(current_node);

        if current_node == end {
            let mut path = vec![];
            let mut current = Some(&current_node);
            while let Some(c) = current {
                current = closed_list.iter().find(|x| Some(x.position) == c.parent);
                path.push(c.position);
            }
            return path.iter().rev().cloned().collect::<Vec<Point>>();
        }

        let children = get_neighbours(&current_node);

        for child in children {
            if !(0..map.len() as i32).contains(&child.position.0)
                || !(0..map[0].len() as i32).contains(&child.position.1)
            {
                continue;
            }
            let current_value =
                map[current_node.position.0 as usize][current_node.position.1 as usize];
            let child_value = map[child.position.0 as usize][child.position.1 as usize];
            if current_value as u8 + 1 < child_value as u8 {
                continue;
            }
            if closed_list.iter().any(|&c| child == c) {
                continue;
            }

            let mut child = child;
            child.g = current_node.g + 1;
            child.h = ((end.position.0 - child.position.0).pow(2) as f64
                + (end.position.1 - child.position.1).pow(2) as f64)
                .sqrt() as i32;
            child.f = child.g + child.h;

            if open_list.iter().any(|&x| x == child && child.g > x.g) {
                continue;
            }

            open_list.insert(child);
        }
    }

    vec![]
}

#[derive(Debug, Clone, Copy)]
struct Node {
    parent: Option<Point>,
    position: Point,
    f: i32,
    g: i32,
    h: i32,
}

impl Node {
    pub fn new(position: Point, parent: Option<Point>) -> Self {
        Self {
            parent,
            position,
            g: 0,
            h: 0,
            f: 0,
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

fn get_neighbours(current: &Node) -> Vec<Node> {
    let neighbours = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    neighbours
        .iter()
        .map(|p| {
            Node::new(
                (current.position.0 + p.0, current.position.1 + p.1),
                Some(current.position),
            )
        })
        .collect_vec()
}

#[test]
fn test() {
    let s = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let got = part1(&generator(s));

    assert_eq!(got, 31);
}

#[test]
fn test2() {
    let s = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let got = part2(&generator(s));

    assert_eq!(got, 29);
}
