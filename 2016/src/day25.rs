use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day25)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.into()).collect_vec()
}

#[aoc(day25, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut output = vec![0, 0];
    let mut counter = 0;

    while !is_ok(&output) {
        let mut memory = HashMap::new();
        memory.insert('a', counter);
        let mut computer = Computer {
            position: 0,
            memory,
        };
        output = computer.run(input);
        counter +=1;
    }

    counter - 1
}

fn is_ok(out: &[i32]) -> bool {
    for i in 0..out.len() - 1 { 
        if out[i] == out[i + 1] {
            return false;
        }
    }

    true
}

pub struct Computer {
    position: usize,
    memory: HashMap<char, i32>,
}

impl Computer {
    fn run(&mut self, instructions: &[Instruction]) -> Vec<i32> {
        let mut output = Vec::new();
        while self.position < instructions.len() && output.len() < 100 {
            match &instructions[self.position] {
                Instruction::Cpy(v1, v2) => {
                    let v1 = match v1 {
                        Operator::Register(k) => self.memory[k],
                        Operator::Value(v) => *v
                    };

                    let r = self.memory.entry(*v2).or_insert(0);
                    *r = v1;
                },
                Instruction::Inc(k) => {
                    let r = self.memory.entry(*k).or_insert(0);
                    *r += 1;
                }
                Instruction::Dec(k) => {
                    let r = self.memory.entry(*k).or_insert(0);
                    *r -= 1;
                }
                Instruction::Jnz(x, y) => {
                    let x = match x {
                        Operator::Register(k) => self.memory[k],
                        Operator::Value(v) => *v
                    };
                    self.position += if x != 0 { *y as usize - 1 } else { 0 };
                },
                Instruction::Out(x) => {
                    let x = match x {
                        Operator::Register(k) => self.memory[k],
                        Operator::Value(v) => *v
                    };
                    output.push(x);
                }
            }

            self.position += 1;
        }

        output
    }
}

pub enum Instruction {
    Cpy(Operator, char),
    Inc(char),
    Dec(char),
    Jnz(Operator, i32),
    Out(Operator)
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
            "out" => Instruction::Out(split[1].into()),
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
