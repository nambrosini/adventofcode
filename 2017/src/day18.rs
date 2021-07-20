use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{From, Into};

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.into()).collect_vec()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Instruction]) -> i64 {
    let mut duet = Duet::new(input.to_vec());
    let mut next = duet.next();

    while next.is_none() {
        next = duet.next();
    }

    next.unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Instruction]) -> i64 {
    let mut duet_1 = DuetSecond::new(input.to_vec(), 0);
    let mut duet_2 = DuetSecond::new(input.to_vec(), 1);

    loop { 
        while let State::Running(v) = duet_1.next().unwrap() {
            if let Some(message) = v {
                duet_2.send(message);
            }
        }

        while let State::Running(v) = duet_2.next().unwrap() {
            if let Some(message) = v {
                duet_1.send(message);
            }
        }

        if (duet_1.state.is_waiting() && !duet_1.has_messages()) || 
            (duet_2.state.is_waiting() && !duet_2.has_messages()) {
            return duet_1.total_messages_count
        }
    }
}

struct Duet {
    instructions: Vec<Instruction>,
    position: usize,
    last_played: Option<i64>,
    map: HashMap<char, i64>,
}

impl Duet {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            position: 0,
            last_played: None,
            map: HashMap::new(),
        }
    }
}

impl Iterator for Duet {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions[self.position].clone();

        match instruction {
            Instruction::Snd(x) => self.last_played = Some(self.map[&x]),
            Instruction::Set(x, y) => {
                self.map.insert(x, y.get_value(&self.map));
            }
            Instruction::Add(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e += val;
            }
            Instruction::Mul(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e *= val;
            }
            Instruction::Mod(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e %= val;
            }
            Instruction::Rcv(x) => {
                if *self.map.entry(x).or_insert(0) != 0 {
                    return self.last_played;
                }
            }
            Instruction::Jgz(x, y) => {
                if *self.map.entry(x).or_insert(0) > 0 {
                    self.position = (self.position as i64 + y.get_value(&self.map)) as usize;
                    return None;
                }
            }
        }

        self.position += 1;

        None
    }
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Running(Option<i64>),
    Waiting
}

impl State {
    fn is_waiting(&self) -> bool {
        match self {
            State::Waiting => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct DuetSecond {
    instructions: Vec<Instruction>,
    position: usize,
    last_played: Option<i64>,
    map: HashMap<char, i64>,
    messages: Vec<i64>,
    other: Option<Box<DuetSecond>>,
    total_messages_count: i64,
    state: State,
}

impl DuetSecond {
    fn new(instructions: Vec<Instruction>, id: i64) -> Self {
        let mut map = HashMap::new();
        map.insert('p', id);
        Self {
            instructions,
            position: 0,
            last_played: None,
            map: map,
            messages: Vec::new(),
            other: None, 
            total_messages_count: 0,
            state: State::Running(None),
        }
    }

    fn get_message(&mut self) -> Option<i64> {
        if self.messages.len() > 0 {
            Some(self.messages.remove(0))
        } else {
            None
        }
    }

    fn send(&mut self, message: i64) {
        self.messages.push(message);
    }

    fn has_messages(&self) -> bool {
        self.messages.len() > 0
    }
}

impl Iterator for DuetSecond {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions[self.position].clone();

        match instruction {
            Instruction::Snd(x) => {
                self.total_messages_count += 1;
                self.position += 1;
                self.state = State::Running(Some(*self.map.entry(x).or_insert(0)));
                return Some(self.state.clone());
            },
            Instruction::Set(x, y) => {
                self.map.insert(x, y.get_value(&self.map));
            }
            Instruction::Add(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e += val;
            }
            Instruction::Mul(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e *= val;
            }
            Instruction::Mod(x, y) => {
                let val = y.get_value(&self.map);
                let e = self.map.entry(x).or_insert(0);
                *e %= val;
            }
            Instruction::Rcv(x) => {
                if let Some(message) = self.get_message() {
                    let e = self.map.entry(x).or_insert(0);
                    *e = message;
                } else {
                    self.state = State::Waiting;
                    return Some(self.state.clone());
                }
            }
            Instruction::Jgz(x, y) => {
                if *self.map.entry(x).or_insert(0) > 0 {
                    self.position = (self.position as i64 + y.get_value(&self.map)) as usize;
                    self.state = State::Running(None);
                    return Some(self.state.clone());
                }
            }
        }

        self.position += 1;

        self.state = State::Running(None);
        Some(self.state.clone())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(char, Value),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let instruction = &s[..3];
        let split = s[4..].split(' ').collect_vec();
        let x: char = split[0].chars().next().unwrap();
        let y: Option<Value> = split.get(1).map(|v| (*v).into());

        match instruction {
            "snd" => Instruction::Snd(x),
            "set" => Instruction::Set(x, y.unwrap()),
            "add" => Instruction::Add(x, y.unwrap()),
            "mul" => Instruction::Mul(x, y.unwrap()),
            "mod" => Instruction::Mod(x, y.unwrap()),
            "rcv" => Instruction::Rcv(x),
            "jgz" => Instruction::Jgz(x, y.unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Register(char),
    Value(i64),
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        if let Ok(x) = s.parse() {
            Value::Value(x)
        } else {
            Value::Register(s.chars().next().unwrap())
        }
    }
}

impl Value {
    fn get_value(&self, registers: &HashMap<char, i64>) -> i64 {
        match self {
            Value::Register(v) => {
                if let Some(val) = registers.get(v) {
                    *val
                } else {
                    0
                }
            }
            Value::Value(x) => *x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

        let s = generator(s);
        assert_eq!(part1(&s), 4);
    }

    #[test]
    fn test2() {
        let s = "snd a
snd b
snd p
rcv a
rcv b
rcv c
rcv d";

        let s = generator(s);
        assert_eq!(part2(&s), 3);
    }

    #[test]
    fn test2_1() {
        let s = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

        let s = generator(s);
        assert_eq!(part1(&s), 1);
    }
}
