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
    let mut res = None;

    while res == None {
        res = duet.run();
    }

    res.unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Instruction]) -> i64 {
    let mut channel1 = vec![];
    let mut channel2 = vec![];

    let mut duet1 = DuetSecond::new(input.to_vec(), 0);
    let mut duet2 = DuetSecond::new(input.to_vec(), 1);

    while duet1.state == State::Running
        || duet2.state == State::Running
        || !channel1.is_empty()
        || !channel2.is_empty()
    {
        duet1.step(&mut channel1, &mut channel2);
        duet2.step(&mut channel2, &mut channel1);
    }

    duet2.total_messages_count
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

impl Duet {
    fn run(&mut self) -> Option<i64> {
        let instruction = self.instructions[self.position];

        match instruction {
            Instruction::Snd(x) => {
                let x = match x {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };
                self.last_played = Some(x);
            }
            Instruction::Set(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e = y;
                }
            }
            Instruction::Add(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e += y;
                }
            }
            Instruction::Mul(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e *= y;
                }
            }
            Instruction::Mod(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e %= y;
                }
            }
            Instruction::Rcv(x) => {
                let x = match x {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                if x != 0 {
                    return self.last_played;
                }
            }
            Instruction::Jgz(x, y) => {
                let x = match x {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                let y = match y {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                if x > 0 {
                    self.position = (self.position as i64 + y - 1) as usize;
                }
            }
        }

        self.position += 1;

        None
    }
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Running,
    Waiting,
}

#[derive(Debug, Clone)]
struct DuetSecond {
    instructions: Vec<Instruction>,
    position: usize,
    map: HashMap<char, i64>,
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
            map,
            total_messages_count: 0,
            state: State::Running,
        }
    }
}

impl DuetSecond {
    fn step(&mut self, sender: &mut Vec<i64>, receiver: &mut Vec<i64>) {
        let instruction = self.instructions[self.position];

        match instruction {
            Instruction::Snd(x) => {
                self.total_messages_count += 1;

                let x = match x {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                sender.push(x);
            }
            Instruction::Set(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e = y;
                }
            }
            Instruction::Add(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e += y;
                }
            }
            Instruction::Mul(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e *= y;
                }
            }
            Instruction::Mod(x, y) => {
                if let Operator::Register(x) = x {
                    let y = match y {
                        Operator::Register(k) => self.map[&k],
                        Operator::Value(v) => v,
                    };

                    let e = self.map.entry(x).or_insert(0);
                    *e %= y;
                }
            }
            Instruction::Rcv(x) => {
                if receiver.is_empty() {
                    self.state = State::Waiting;
                    return;
                } else if let Operator::Register(x) = x {
                    let message = receiver.remove(0);
                    let e = self.map.entry(x).or_insert(0);
                    *e = message;
                    self.state = State::Running;
                }
            }
            Instruction::Jgz(x, y) => {
                let x = match x {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                let y = match y {
                    Operator::Register(k) => self.map[&k],
                    Operator::Value(v) => v,
                };

                if x > 0 {
                    self.position = (self.position as i64 + y - 1) as usize;
                }
            }
        }

        self.position += 1;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Snd(Operator),
    Set(Operator, Operator),
    Add(Operator, Operator),
    Mul(Operator, Operator),
    Mod(Operator, Operator),
    Rcv(Operator),
    Jgz(Operator, Operator),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let s = s.split(' ').collect_vec();
        match s[0] {
            "snd" => Instruction::Snd(s[1].into()),
            "set" => Instruction::Set(s[1].into(), s[2].into()),
            "add" => Instruction::Add(s[1].into(), s[2].into()),
            "mul" => Instruction::Mul(s[1].into(), s[2].into()),
            "mod" => Instruction::Mod(s[1].into(), s[2].into()),
            "rcv" => Instruction::Rcv(s[1].into()),
            "jgz" => Instruction::Jgz(s[1].into(), s[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Register(char),
    Value(i64),
}

impl From<&str> for Operator {
    fn from(s: &str) -> Operator {
        if let Ok(v) = s.parse::<i64>() {
            Operator::Value(v)
        } else {
            Operator::Register(s.chars().next().unwrap())
        }
    }
}

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
    let s = generator(
        "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d",
    );
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
    assert_eq!(part2(&s), 1);
}
