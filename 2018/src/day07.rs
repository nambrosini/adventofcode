use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<(char, char)> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = re.captures_iter(l).next().unwrap();

            (
                cap[1].chars().next().unwrap(),
                cap[2].chars().next().unwrap(),
            )
        })
        .collect_vec()
}

// #[aoc(day07, part1)]
// pub fn part1(input: &[(char, char)]) -> String {
//     let mut map: HashMap<char, Vec<char>> = HashMap::new();

//     for (key, connection) in input {
//         let e = map.entry(*key).or_insert_with(Vec::new);
//         e.push(*connection);
//     }

//     let mut queue = map
//         .keys()
//         .filter(|&x| !map.values().flatten().any(|y| x == y))
//         .copied()
//         .collect_vec();

//     queue.sort_unstable();

//     let mut res = String::new();

//     while queue.len() > 0 {
//         let value = queue.remove(0);
//         res.push(value);
//         if let Some(children) = map.get(&value) {
//             queue.append(&mut children.clone());
//         }
//         queue.sort_unstable();
//     }

//     res.chars()
//         .rev()
//         .unique()
//         .collect_vec()
//         .iter()
//         .rev()
//         .join("")
// }

#[aoc(day07, part1)]
pub fn part1(input: &Vec<(char, char)>) -> String {
    let mut map: HashMap<char, Vec<char>> = HashMap::new();

    for (key, connection) in input {
        let e = map.entry(*key).or_insert_with(Vec::new);
        e.push(*connection);
    }

    let mut queue = PriorityQueue::new(map
        .keys()
        .filter(|&x| !map.values().flatten().any(|y| x == y))
        .map(|x| Node::new(*x))
        .collect_vec());

    queue.sort();

    let mut res = String::new();

    while queue.len() > 0 {
        let value = queue.remove(0);
        res.push(value.value);
        if let Some(children) = map.get(&value.value) {
            queue.append(children.clone());
        }
        queue.sort();
    }

    res.chars()
        .rev()
        .unique()
        .collect_vec()
        .iter()
        .rev()
        .join("")
}

struct PriorityQueue {
    queue: Vec<Node>
}

impl PriorityQueue {
    fn new(queue: Vec<Node>) -> Self {
        Self {
            queue
        }
    }

    fn push(&mut self, value: char) {
        for n in self.queue.iter_mut() {
            if n.value == value {
                n.increase_priority();
                return;
            }
        }

        self.queue.push(Node::new(value));
    }

    fn append(&mut self, values: Vec<char>) {
        for c in values {
            self.push(c);
        }
    }

    fn sort(&mut self) {
        self.queue.sort();
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn remove(&mut self, index: usize) -> Node {
        self.queue.remove(index)
    }
}

#[derive(Eq)]
struct Node {
    value: char,
    priority: usize,
}

impl Node {
    fn new(value: char) -> Self {
        Self {
            value,
            priority: 1
        }
    }
    fn increase_priority(&mut self) {
        self.priority += 1;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.priority < other.priority || self.priority == other.priority && self.value < other.value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.priority == other.priority
    }
}


#[test]
fn test1() {
    let s = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    let s = generator(s);

    assert_eq!(&part1(&s), "CABDFE");
}