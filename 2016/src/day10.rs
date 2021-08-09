use std::{cmp, collections::HashMap, vec};
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|l| l.into())
        .collect_vec()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Operation]) -> usize {
    let mut input = input.to_vec();
    let mut robots: HashMap<usize, Bot> = HashMap::new();
    let v1 = 61;
    let v2 = 17;
    let mut count = 0;

    let mut output: HashMap<usize, usize> = HashMap::new();

    while input.len() > 0 {
        let mut new_input: Vec<Operation> = vec![];

        for op in input.iter() {
            match op {
                Operation::Gets(id, v) => {
                    let b = robots.entry(*id).or_insert_with(Bot::new);
                    if !b.gets(*v) {
                        new_input.push(*op);
                        continue;
                    }

                    if b.has_values(v1, v2) {
                        return *id;
                    }
                },
                Operation::Gives(id, rec1, rec2) => {
                    let mut b: Bot = if let Some(b) = robots.get(id) {
                        if b.has_values(v1, v2) {
                            return *id;
                        }
                        if !b.has_values_assigned() {
                            new_input.push(*op);
                            continue;
                        }
                        (*b).clone()
                    } else {
                        new_input.push(*op);
                        continue;
                    };

                    match rec1 {
                        Receiver::Bot(idr) => {
                            let r = robots.entry(*idr).or_insert_with(Bot::new);
                            if !b.gives(r, Value::Low) {
                                new_input.push(*op);
                                continue;
                            }
                        },
                        Receiver::Output(idr) => {
                            if let Some(low) = b.take(Value::Low) {
                                output.insert(*idr, low);
                            } else {
                                new_input.push(*op)
                            }
                        },
                    }

                    match rec2 {
                        Receiver::Bot(idr) => {
                            let r = robots.entry(*idr).or_insert_with(Bot::new);
                            if !b.gives(r, Value::High) {
                                new_input.push(*op);
                                continue;
                            }
                        },
                        Receiver::Output(idr) => {
                            if let Some(high) = b.take(Value::High) {
                                output.insert(*idr, high);
                            } else {
                                new_input.push(*op)
                            }
                        },
                    }

                    robots.insert(*id, b);
                },
            }
        }

        count += 1;

        if new_input.len() == input.len() {
            panic!("input has not changed: {} == {}, count = {}", input.len(), new_input.len(), count)
        }

        input = new_input.clone();
    }

    unreachable!()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Operation]) -> usize {
    let mut input = input.to_vec();
    let mut robots: HashMap<usize, Bot> = HashMap::new();
    let mut count = 0;

    let mut output: HashMap<usize, usize> = HashMap::new();

    while input.len() > 0 {
        let mut new_input: Vec<Operation> = vec![];

        for op in input.iter() {

            if output.contains_key(&0) && output.contains_key(&1) && output.contains_key(&2) {
                return output[&0] * output[&1] * output[&2];
            }
            match op {
                Operation::Gets(id, v) => {
                    let b = robots.entry(*id).or_insert_with(Bot::new);
                    if !b.gets(*v) {
                        new_input.push(*op);
                        continue;
                    }
                },
                Operation::Gives(id, rec1, rec2) => {
                    let mut b: Bot = if let Some(b) = robots.get(id) {
                        if !b.has_values_assigned() {
                            new_input.push(*op);
                            continue;
                        }
                        (*b).clone()
                    } else {
                        new_input.push(*op);
                        continue;
                    };

                    match rec1 {
                        Receiver::Bot(idr) => {
                            let r = robots.entry(*idr).or_insert_with(Bot::new);
                            if !b.gives(r, Value::Low) {
                                new_input.push(*op);
                                continue;
                            }
                        },
                        Receiver::Output(idr) => {
                            if let Some(low) = b.take(Value::Low) {
                                output.insert(*idr, low);
                            } else {
                                new_input.push(*op)
                            }
                        },
                    }

                    match rec2 {
                        Receiver::Bot(idr) => {
                            let r = robots.entry(*idr).or_insert_with(Bot::new);
                            if !b.gives(r, Value::High) {
                                new_input.push(*op);
                                continue;
                            }
                        },
                        Receiver::Output(idr) => {
                            if let Some(high) = b.take(Value::High) {
                                output.insert(*idr, high);
                            } else {
                                new_input.push(*op)
                            }
                        },
                    }

                    robots.insert(*id, b);
                },
            }
        }

        count += 1;

        if new_input.len() == input.len() {
            panic!("input has not changed: {} == {}, count = {}", input.len(), new_input.len(), count)
        }

        input = new_input.clone();
    }

    output[&0] * output[&1] * output[&2]
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    // Gets(id, val)
    Gets(usize, usize),
    // Gives(id, low_id, high_id)
    Gives(usize, Receiver, Receiver)
}

#[derive(Debug, Clone, Copy)]
pub enum Receiver {
    Bot(usize),
    Output(usize)
}

impl From<(&str, &str)> for Receiver {
    fn from(s: (&str, &str)) -> Self {
        match s.0 {
            "bot" => Self::Bot(s.1.parse().unwrap()),
            "output" => Self::Output(s.1.parse().unwrap()),
            _ => unreachable!()
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        if s.starts_with("value") {
            // Gets
            let re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
            let cap = re.captures_iter(s).next().unwrap();

            Self::Gets(cap[2].parse().unwrap(), cap[1].parse().unwrap())
        } else {
            // Gives
            let re = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();

            let cap = re.captures_iter(s).next().unwrap();

            let id: usize = cap[1].parse().unwrap();
            let receiver1: Receiver = (&cap[2], &cap[3]).into();
            let receiver2: Receiver = (&cap[4], &cap[5]).into();

            Self::Gives(id, receiver1, receiver2)
        }
    }
}

pub enum Value{
    Low,
    High
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    a: Option<usize>,
    b: Option<usize>,
}

impl Bot {
    fn new() -> Self {
        Bot {
            a: None,
            b: None
        }
    }

    fn gets(&mut self, value: usize) -> bool {
        if self.a.is_none() {
            self.a = Some(value);
            return true;
        } else if self.b.is_none() {
            self.b = Some(value);
            return true;
        }
        return false;
    }

    fn gives(&mut self, other: &mut Self, value: Value) -> bool {
        if self.a.is_none() && self.b.is_none() {
            return false;
        }

        if self.a.is_some() && self.b.is_none() {
            return other.gets(self.a.take().unwrap());
        }

        if self.b.is_some() && self.a.is_none() {
            return other.gets(self.b.take().unwrap());
        }

        match value {
            Value::Low => {
                match self.a.unwrap().cmp(&self.b.unwrap()) {
                    cmp::Ordering::Less => {
                        let a = self.a.take().unwrap();
                        if other.gets(a) {
                            return true;
                        }
                    },
                    cmp::Ordering::Greater => {
                        let b = self.b.take().unwrap();
                        if other.gets(b) {
                            return true;
                        }
                    },
                    _ => unreachable!()
                }
            },
            Value::High => {
                match self.a.unwrap().cmp(&self.b.unwrap()) {
                    cmp::Ordering::Less => {
                        let b = self.b.take().unwrap();
                        if other.gets(b) {
                            return true;
                        }
                    },
                    cmp::Ordering::Greater => {
                        let a = self.a.take().unwrap();
                        if other.gets(a) {
                            return true;
                        }
                    },
                    _ => unreachable!()
                }
            },
        }

        false
    }

    fn take(&mut self, value: Value) -> Option<usize> {
        if !self.has_values_assigned() {
            return None;
        }
        match value {
            Value::Low => if self.a.unwrap() < self.b.unwrap() {
                self.a.take()
            } else {
                self.b.take()
            },
            Value::High => if self.b.unwrap() < self.a.unwrap() {
                self.b.take()
            } else {
                self.a.take()
            },
        }
    }

    fn has_values(&self, v1: usize, v2: usize) -> bool {
        if let Some(a) = self.a {
            if let Some(b) = self.b {
                if a == v1 && b == v2 || a == v2 && b == v1 {
                    return true;
                }
            }
        }
        false
    }
    
    fn has_values_assigned(&self) -> bool {
        self.a.is_some() && self.b.is_some()
    }
}