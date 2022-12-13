use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Grid<char> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Grid<char>) -> usize {
    let start = find_character(input, 'S')[0];
    let end = find_character(input, 'E')[0];
    let mut input = input.to_vec();

    input[start.0 as usize][start.1 as usize] = 'a';
    input[end.0 as usize][end.1 as usize] = 'z';

    a_star(&input, &start, &end, heuristics, neighbors).unwrap().len() - 1
}

#[aoc(day12, part2)]
pub fn part2(input: &Grid<char>) -> usize {
    let mut start = find_character(input, 'a');
    start.append(&mut find_character(input, 'S'));

    let end = find_character(input, 'E')[0];

    let mut input = input.to_vec();

    input[start[0].0 as usize][start[0].1 as usize] = 'a';
    input[end.0 as usize][end.1 as usize] = 'z';

    start
        .iter()
        .map(|start|a_star(&input, start, &end, heuristics, neighbors).map(|path| path.len()))
        .filter(|res| res.is_some())
        .min()
        .unwrap()
        .unwrap()
        - 1
}

fn heuristics(current: &Position, end: &Position) -> i32 {
    ((end.0 - current.0).pow(2) as f64 + (end.1 - current.1).pow(2) as f64).sqrt() as i32
}

fn neighbors(grid: &Grid<char>, current: &Position) -> Vec<Position> {
    let v = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    v.iter()
        .map(|p| (p.0 + current.0, p.1 + current.1))
        .filter(|p| (0..grid.len() as i32).contains(&p.0) && (0..grid[0].len() as i32).contains(&p.1))
        .filter(|p| grid[current.0 as usize][current.1 as usize] as u8 + 1 >= grid[p.0 as usize][p.1 as usize] as u8)
        .collect()
}

fn find_character(map: &Grid<char>, searched: char) -> Vec<Position> {
    map.iter()
        .flatten()
        .enumerate()
        .filter(|(_, c)| c == &&searched)
        .map(|(i, _)| {
            (
                i as i32 / map[0].len() as i32,
                i as i32 % map[0].len() as i32,
            )
        })
        .collect()
}

pub type Grid<T> = Vec<Vec<T>>;
pub type Position = (i32, i32);

#[derive(Clone)]
struct Node {
    parent: Option<Box<Node>>,
    position: Position,
    g: i32,
    h: i32,
    f: i32,
}

impl Node {
    fn new(position: Position, parent: Option<Box<Node>>) -> Self {
        Self {
            parent,
            position,
            g: 0,
            h: 0,
            f: 0
        }
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.f.cmp(&self.f))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn a_star<T>(
    grid: &Grid<T>,
    start: &Position,
    end: &Position,
    heuristics: fn(&Position, &Position) -> i32,
    neighbors: fn(&Grid<T>, &Position) -> Vec<Position>,
) -> Option<Vec<Position>> {
    let mut open = BinaryHeap::new();
    open.push(Node {
        parent: None,
        position: *start,
        g: 0,
        h: 0,
        f: 0,
    });
    open.push(Node::new(*start, None));
    let mut closed: Vec<Node> = vec![];

    while let Some(current) = open.pop() {
        if closed.iter().any(|n| n.position == current.position) {
            continue;
        }
        closed.push(current.clone());

        if &current.position == end {
            let mut path: Vec<Position> = vec![];
            let mut current: Option<Box<Node>> = Some(Box::new(current));

            while let Some(n) = current {
                path.insert(0, n.as_ref().position);
                current = n.parent.clone();
            }

            return Some(path);
        }

        let neighbors = neighbors(grid, &current.position);

        for child in neighbors {
            let mut child = Node::new(child, Some(Box::new(current.clone())));
            child.g = current.g + 1;
            child.h = heuristics(&current.position, end);
            child.f = child.g + child.h;

            if open.iter().any(|n| n.position == child.position && child.g > n.g) {
                continue;
            }

            if let Some(n) = closed.iter_mut().find(|node| node.position == child.position) {
                if &child < n {
                    n.parent = child.parent.clone();
                    n.g = child.g;
                    n.h = child.h;
                    n.f = child.f;
                }
            }

            open.push(child);
        }
    }

    None
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
