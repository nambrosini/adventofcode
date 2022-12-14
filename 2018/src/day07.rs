use std::{collections::HashMap, vec};

const WORKERS_NUM: usize = 5;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for l in input.lines() {
        let split: Vec<&str> = l.split_whitespace().collect();

        let first = split[1].to_string();
        let second = split[7].to_string();

        let entry: &mut Vec<String> = map.entry(first).or_default();
        entry.push(second);
    }

    map
}

#[aoc(day07, part1)]
pub fn part1(map: &HashMap<String, Vec<String>>) -> String {
    let mut map = map.clone();

    let mut queue = find_start(&map);
    queue.sort_by(|a, b| b.cmp(a));
    let mut done: Vec<String> = vec![];

    while let Some(x) = queue.pop() {
        if done.contains(&x) || map.values().flatten().any(|v| v == &x) {
            continue;
        }
        done.push(x.clone());

        if let Some(v) = map.get(&x) {
            let mut v = v.to_vec();
            queue.append(&mut v);
            map.remove(&x);
        }

        queue.sort_by(|a, b| b.cmp(a));
    }

    done.join("")
}

#[aoc(day07, part2)]
fn part2(map: &HashMap<String, Vec<String>>) -> usize {
    let mut map = map.clone();
    let mut queue = find_start(&map);
    queue.sort_by(|a, b| b.cmp(a));

    let mut done: Vec<String> = vec![];
    let mut workers: Vec<Worker> = vec![];

    let mut ticks = 0;

    while !map.is_empty() || !queue.is_empty() || !workers.is_empty() {
        let mut index_remove: Vec<usize> = vec![];

        for (i, w) in workers.iter_mut().enumerate() {
            if let Some(letter) = w.step() {
                done.push(letter.to_string());
                if let Some(v) = map.get(&letter) {
                    let mut v = v.to_vec();
                    queue.append(&mut v);
                    map.remove(&letter);
                }
                index_remove.push(i);
                queue.sort_by(|a, b| b.cmp(a));
            }
        }

        for i in &index_remove {
            workers.remove(*i);
        }

        while workers.len() < WORKERS_NUM {
            if let Some(x) = queue.pop() {
                if done.contains(&x)
                    || map.values().flatten().any(|v| v == &x)
                    || workers.iter().map(|w| w.letter.clone()).any(|l| l == x)
                {
                    continue;
                }
                workers.push(Worker::new(x));
            } else {
                break;
            }
        }
        ticks += 1;
    }

    ticks - 1
}

#[derive(Debug)]
struct Worker {
    letter: String,
    max_time: usize,
    tick: usize,
}

impl Worker {
    fn new(letter: String) -> Self {
        let max_time = 60 + letter.chars().next().unwrap() as usize - 64;
        Self {
            letter,
            max_time,
            tick: 0,
        }
    }

    fn step(&mut self) -> Option<String> {
        self.tick += 1;
        if self.tick >= self.max_time {
            Some(self.letter.clone())
        } else {
            None
        }
    }
}

fn find_start(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut start = vec![];

    for k in map.keys() {
        if !map.values().flatten().any(|v| v == k) {
            start.push(k.to_string());
        }
    }

    start
}

#[test]
pub fn test() {
    let s = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!("CABDFE", &part1(&generator(s)));
}

#[test]
pub fn test2() {
    let s = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!(253, part2(&generator(s)));
}
