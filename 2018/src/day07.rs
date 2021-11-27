use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::{Ord, Ordering};
use std::fmt;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<(char, char)> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let mut vec = Vec::new();

    for caps in re.captures_iter(input) {
        let new = (
            caps[1].chars().next().unwrap(),
            caps[2].chars().next().unwrap(),
        );

        vec.push(new);
    }

    vec
}

#[aoc(day07, part1)]
pub fn part1(input: &[(char, char)]) -> String {
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

    while let Some(value) = queue.remove(0) {
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

#[aoc(day07, part2)]
fn part2(input: &[(char, char)]) -> usize {
    let mut time_passed = 0;

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

    println!("{:?}", queue.queue);

    queue.sort();

    let mut workers: Vec<(Option<Node>, usize, usize)> = [(None, 0, 0); 5].to_vec();

    println!("Second\tWorker 1\tWorker 2\tWorker 3\tWorker 4\tWorker 5\tDone");

    let mut dones = vec![];
    loop {
        for worker in workers.iter_mut() {
            if let Some(node) = worker.0 {
                worker.1 += 1;
                if worker.1 == worker.2 {
                    dones.push(node.value);
                    if let Some(children) = map.get(&node.value) {
                        queue.append(children.clone());
                    }
                    if let Some(node) = queue.remove(0) {
                        worker.0 = Some(node);
                        worker.1 = 0;
                        worker.2 = (node.value as u8 - 4) as usize;
                    } else {
                        worker.0 = None;
                        worker.1 = 0;
                        worker.2 = 0;
                    }
                }
            } else if let Some(node) = queue.remove(0) {
                worker.0 = Some(node);
                worker.1 = 0;
                worker.2 = (node.value as u8 - 4) as usize;
            }
        }

        if all_workers_finished(&workers) && queue.len() == 0 {
            break;
        }
        let worker0 = if let Some(node) = workers[0].0 {
            node.value
        } else {
            '.'
        };
        let worker1 = if let Some(node) = workers[1].0 {
            node.value
        } else {
            '.'
        };
        let worker2 = if let Some(node) = workers[2].0 {
            node.value
        } else {
            '.'
        };
        let worker3 = if let Some(node) = workers[3].0 {
            node.value
        } else {
            '.'
        };
        let worker4 = if let Some(node) = workers[4].0 {
            node.value
        } else {
            '.'
        };
        println!("{}\t\t   {}\t\t   {}\t\t   {}\t\t   {}\t\t   {}\t\t{}", time_passed, worker0, worker1, worker2, worker3, worker4, dones.iter().join(""));
        time_passed += 1;
    }

    time_passed
}

fn all_workers_finished(workers: &[(Option<Node>, usize, usize)]) -> bool {
    let mut finished = true;

    for (_, x, y) in workers {
        if x != y {
            finished = false;
        }
    }

    finished
}

#[derive(Debug)]
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

    fn remove(&mut self, index: usize) -> Option<Node> {
        if self.queue.is_empty() {
            return None;
        }
        Some(self.queue.remove(index))
    }
}

#[derive(Debug, Eq, Clone, Copy)]
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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
    assert_eq!(part2(&s), 1);
}