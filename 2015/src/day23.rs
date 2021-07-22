use std::convert::{From, Into};
use itertools::Itertools;
use std::fmt;

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| l.into())
        .collect_vec()
}

#[aoc(day23, part1)]
pub fn part1(input: &[Instruction]) -> i128 {
    println!("{}", input.iter().join("\n"));
    let mut processor = Processor::new([0, 0], input);
    processor.run()
}

#[aoc(day23, part2)]
pub fn part2(input: &[Instruction]) -> i128 {
    println!("{}", input.iter().join("\n"));
    let mut processor = Processor::new([1, 0], input);
    processor.run()
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i128),
    Jie(usize, i128),
    Jio(usize, i128),
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Hlf(r) => write!(fmt, "hlf {}", r),
            Instruction::Tpl(r) => write!(fmt, "tpl {}", r),
            Instruction::Inc(r) => write!(fmt, "inc {}", r),
            Instruction::Jmp(o) => write!(fmt, "jmp {}", o),
            Instruction::Jie(r, o) => write!(fmt, "jie {}, {}", r, o),
            Instruction::Jio(r, o) => write!(fmt, "jio {}, {}", r, o),
        }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let instruction = &s[..3];
        let other = &s[4..];

        match instruction {
            "hlf" => Instruction::Hlf(if other == "a" { 0 } else if other == "b" { 1 } else { unreachable!() }),
            "tpl" => Instruction::Tpl(if other == "a" { 0 } else if other == "b" { 1 } else { unreachable!() }),
            "inc" => Instruction::Inc(if other == "a" { 0 } else if other == "b" { 1 } else { unreachable!() }),
            "jmp" => {
                Instruction::Jmp(other.parse().unwrap())
            },
            "jie" => {
                let split: Vec<&str> = other.split(", ").collect();
                let register = if split[0] == "a" { 0 } else if other == "b" { 1 } else { unreachable!() };
                let offset = split[1].parse().unwrap();
                Instruction::Jie(register, offset)
            },
            "jio" => {
                let split: Vec<&str> = other.split(", ").collect();
                let register = if split[0] == "a" { 0 } else if other == "b" { 1 } else { unreachable!() };
                let offset = split[1].parse().unwrap();
                Instruction::Jio(register, offset)
            },
            _ => unreachable!(),
        }
    }
}

struct Processor {
    registers: [i128; 2],
    instructions: Vec<Instruction>
}

impl Processor {
    fn new(registers: [i128; 2], instructions: &[Instruction]) -> Self {
        Self {
            registers: registers,
            instructions: instructions.to_vec()
        }
    }

    fn run(&mut self) -> i128 {
        let mut position: i128 = 0;

        while position < self.instructions.len() as i128 {
            match self.instructions[position as usize] {
                Instruction::Hlf(r) => {
                    self.registers[r] /= 2;
                    position += 1;
                },
                Instruction::Tpl(r) => {
                    self.registers[r] *= 3;
                    position += 1;
                },
                Instruction::Inc(r) => {
                    self.registers[r] += 1;
                    position += 1;
                },
                Instruction::Jmp(o) => {
                    position += o;
                },
                Instruction::Jie(r, o) => {
                    if self.registers[r] % 2 == 0 {
                        position += o;
                    } else {
                        position += 1;
                    }
                },
                Instruction::Jio(r, o) => {
                    if self.registers[r] == 1 {
                        position += o;
                    } else { 
                        position += 1;
                    }
                },
            }
            
        }

        self.registers[1]
    }
}

#[test]
fn test() {
    let s = "inc b
jio b, +2
tpl b
inc b";

    let s = generator(s);
    assert_eq!(part1(&s), 2);
}