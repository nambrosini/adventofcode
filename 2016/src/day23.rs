use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.into()).collect_vec()
}

#[aoc(day23, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut memory = HashMap::new();
    memory.insert('a', 7);
    let mut computer = Computer {
        position: 0,
        memory
    };

    computer.run(input)
}

#[aoc(day23, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut memory = HashMap::new();
    memory.insert('a', 12);
    let mut computer = Computer {
        position: 0,
        memory
    };

    computer.run(input)
}

pub struct Computer {
    position: usize,
    memory: HashMap<char, i32>,
}

impl Computer {
    fn run(&mut self, instructions: &[Instruction]) -> i32 {
        let mut instructions: Vec<Instruction> = instructions.to_vec();

        while self.position < instructions.len() {
            match &instructions[self.position] {
                Instruction::Cpy(v1, v2) => {
                    if let Operator::Register(v2) = v2 {
                        let v1 = match v1 {
                            Operator::Register(k) => self.memory[k],
                            Operator::Value(v) => *v
                        };

                        let r = self.memory.entry(*v2).or_insert(0);
                        *r = v1;
                    }
                },
                Instruction::Inc(k) => {
                    if let Operator::Register(k) = k {
                        let r = self.memory.entry(*k).or_insert(0);
                        *r += 1;
                    }
                }
                Instruction::Dec(k) => {
                    if let Operator::Register(k) = k {
                        let r = self.memory.entry(*k).or_insert(0);
                        *r -= 1;
                    }
                }
                Instruction::Jnz(x, y) => {
                    let x = match x {
                        Operator::Register(k) => self.memory[k],
                        Operator::Value(v) => *v
                    };
                    let y = match y {
                        Operator::Register(k) => self.memory[k],
                        Operator::Value(v) => *v
                    };
                    self.position += if x != 0 { y as usize - 1 } else { 0 };
                },
                Instruction::Tgl(v) => {
                    let position: usize = self.position + match v {
                        Operator::Value(v) => *v,
                        Operator::Register(v) => self.memory[v]
                    } as usize;

                    if (0..instructions.len()).contains(&position) {
                        let ins = instructions[position];

                        let new_instruction = match ins {
                            Instruction::Inc(v) => Instruction::Dec(v), 
                            Instruction::Dec(v) => Instruction::Inc(v),
                            Instruction::Tgl(v) => Instruction::Inc(v),
                            Instruction::Jnz(v1, v2) => Instruction::Cpy(v1, v2),
                            Instruction::Cpy(v1, v2) => Instruction::Jnz(v1, v2)
                        };

                        instructions[position] = new_instruction;
                    }
                }
            }
            self.position += 1;
        }

        self.memory[&'a']
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Cpy(Operator, Operator),
    Inc(Operator),
    Dec(Operator),
    Jnz(Operator, Operator),
    Tgl(Operator)
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Register(char),
    Value(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split = s.split(' ').collect_vec();

        match split[0] {
            "cpy" => Instruction::Cpy(split[1].into(), split[2].into()),
            "inc" => Instruction::Inc(split[1].into()),
            "dec" => Instruction::Dec(split[1].into()),
            "jnz" => Instruction::Jnz(split[1].into(), split[2].into()),
            "tgl" => Instruction::Tgl(split[1].into()),
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

#[test]
fn test11() {
    let s = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    let s = generator(s);
    assert_eq!(part1(&s), 3);
}