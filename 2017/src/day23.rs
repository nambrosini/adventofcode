use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[aoc(day23, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut tablet = Tablet::new(input);

    tablet.run()
}

#[aoc(day23, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut b = 105700;

    for i in (0..b).step(17) {
        println!("{}", i);
    }

    0
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub struct Tablet {
    instructions: Vec<Instruction>,
    memory: [i32; 8]
}

impl Tablet {
    fn new(input: &[Instruction]) -> Tablet {
        Self { 
            instructions: input.to_vec(),
            memory: [0; 8]
        }
    }

    fn run(&mut self) -> usize {
        let mut position = 0;
        let mut mults = 0;

        while position < self.instructions.len() {
            match self.instructions[position] {
                Instruction::Set(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] = y;
                    }
                    position += 1;
                },
                Instruction::Sub(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] -= y;
                    }
                    position += 1;
                },
                Instruction::Mul(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] *= y;
                    }
                    position += 1;
                    mults += 1;
                },
                Instruction::Jnz(x, y) => {
                    let x = self.mem_get(x);
                    let y = self.mem_get(y);

                    if x != 0 {
                        position += y as usize;
                    } else {
                        position += 1;
                    }
                }
            }
        }

        mults
    }

    fn mem_get(&self, p: Param) -> i32 {
        match p {
            Param::Register(r) => {
                if let Some(x) = self.memory.get(Self::get_index(r)) {
                    *x
                } else {
                    0
                }
            },
            Param::Value(v) => v
        }
    }

    fn get_index(register: char) -> usize {
        (register as u8 - 97u8) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Set(Param, Param),
    Sub(Param, Param),
    Mul(Param, Param),
    Jnz(Param, Param)
}

impl From<&str> for Instruction {
    fn from(c: &str) -> Instruction {
        let split = c.split(' ').collect_vec();

        match split[0] {
            "set" => Instruction::Set(split[1].into(), split[2].into()),
            "sub" => Instruction::Sub(split[1].into(), split[2].into()),
            "mul" => Instruction::Mul(split[1].into(), split[2].into()),
            "jnz" => Instruction::Jnz(split[1].into(), split[2].into()),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Param {
    Register(char),
    Value(i32)
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        match s.parse::<i32>() {
            Ok(v) => Param::Value(v),
            Err(_) => Param::Register(s.chars().next().unwrap())
        }
    }
}