use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.into()).collect_vec()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut computer = Computer {
        position: 0,
        memory: HashMap::new(),
    };

    computer.run(input)
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut memory = HashMap::new();
    memory.insert('c', 1);

    let mut computer = Computer {
        position: 0,
        memory,
    };

    computer.run(input)
}

pub struct Computer {
    position: usize,
    memory: HashMap<char, i32>,
}

impl Computer {
    fn run(&mut self, instructions: &[Instruction]) -> i32 {
        while self.position < instructions.len() {
            match &instructions[self.position] {
                Instruction::Cpy(v1, v2) => match v1 {
                    Operator::Register(k) => {
                        let v: i32 = self.memory[k];
                        let r = self.memory.entry(*v2).or_insert(0);
                        *r = v;
                    }
                    Operator::Value(v) => {
                        let r = self.memory.entry(*v2).or_insert(0);
                        *r = *v;
                    }
                },
                Instruction::Inc(k) => {
                    let r = self.memory.entry(*k).or_insert(0);
                    *r += 1;
                }
                Instruction::Dec(k) => {
                    let r = self.memory.entry(*k).or_insert(0);
                    *r -= 1;
                }
                Instruction::Jnz(o, v) => {
                    self.position = (self.position as i32
                        + match o {
                            Operator::Register(k) => {
                                if let Some(&r) = self.memory.get(k) {
                                    if r != 0 {
                                        *v - 1
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                }
                            }
                            Operator::Value(v1) => {
                                if v1 != &0 {
                                    *v - 1
                                } else {
                                    0
                                }
                            }
                        }) as usize;
                }
            }

            self.position += 1;
        }

        self.memory[&'a']
    }
}

pub enum Instruction {
    Cpy(Operator, char),
    Inc(char),
    Dec(char),
    Jnz(Operator, i32),
}

pub enum Operator {
    Register(char),
    Value(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split = s.split(' ').collect_vec();

        match split[0] {
            "cpy" => Instruction::Cpy(split[1].into(), split[2].parse().unwrap()),
            "inc" => Instruction::Inc(split[1].chars().next().unwrap()),
            "dec" => Instruction::Dec(split[1].chars().next().unwrap()),
            "jnz" => Instruction::Jnz(split[1].into(), split[2].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"(-\d+|\d+)").unwrap();

        if re.is_match(s) {
            Operator::Value(s.parse().unwrap())
        } else {
            Operator::Register(s.chars().next().unwrap())
        }
    }
}

#[test]
fn test1() {
    let s = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    let s = generator(s);

    assert_eq!(part1(&s), 42);
}
