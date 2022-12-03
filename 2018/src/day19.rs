#[aoc_generator(day19)]
pub fn generator(input: &str) -> (usize, Vec<Instruction>) {
    let lines: Vec<&str> = input.lines()
        .collect();
    let ip = lines[0].split_whitespace().last().unwrap().parse().unwrap();
    let instructions = lines.iter().skip(1).map(|&l| l.into()).collect();

    (ip, instructions)
}

#[aoc(day19, part1)]
pub fn part1(input: &(usize, Vec<Instruction>)) -> usize {
    let mut device = Device::new(input.0);
    device.simulate(&input.1);
    device.regs[0]
}

pub struct Instruction {
    op_code: OpCode,
    values: Vec<usize>
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        let op_code = split[0].into();
        let values = split.iter().skip(1).map(|l| l.parse().unwrap()).collect();
        Self {
            op_code,
            values
        }
    }
}

struct Device {
    regs: Vec<usize>,
    ip: usize
}

impl Device {
    fn new(ip: usize) -> Self {
        Self {
            ip,
            ..Default::default()
        }
    }

    fn simulate(&mut self, instructions: &[Instruction]) {
        while self.ip < instructions.len() {
            if self.ip == 34 {
                println!("{:?}", self.regs);
            }
            self.regs[0] = self.ip;
            self.exec_instruction(instructions[self.ip].op_code, instructions[self.ip].values.clone());
            self.ip = self.regs[0] + 1;
        }
    }

    fn exec_instruction(&mut self, op_code: OpCode, instruction: Vec<usize>) {
        match op_code {
            OpCode::Addr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = a + b;
            },
            OpCode::Addi => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = a + b;
            },
            OpCode::Mulr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = a * b;
            },
            OpCode::Muli => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = a * b;
            },
            OpCode::Banr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = a & b;
            },
            OpCode::Bani => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = a & b;
            },
            OpCode::Borr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = a | b;
            },
            OpCode::Bori => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = a | b;
            },
            OpCode::Setr => {
                let a = self.regs[instruction[0]];
                self.regs[instruction[2]] = a;
            },
            OpCode::Seti => {
                let a = instruction[0];
                self.regs[instruction[2]] = a;
            },
            OpCode::Gtir => {
                let a = instruction[0];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = (a > b) as usize;
            },
            OpCode::Gtri => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = (a > b) as usize;
            },
            OpCode::Gtrr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = (a > b) as usize;
            },
            OpCode::Eqir => {
                let a = instruction[0];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = (a == b) as usize;
            },
            OpCode::Eqri => {
                let a = self.regs[instruction[0]];
                let b = instruction[1];
                self.regs[instruction[2]] = (a == b) as usize;
            },
            OpCode::Eqrr => {
                let a = self.regs[instruction[0]];
                let b = self.regs[instruction[1]];
                self.regs[instruction[2]] = (a == b) as usize;
            },
        }
    }
}

impl Default for Device {
    fn default() -> Self {
        Self {
            regs: vec![0; 6],
            ip: 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl From<&str> for OpCode {
    fn from(s: &str) -> Self {
        match s {
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "boor" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => {
                println!("{}", s);
                unreachable!()
            }
        }
    }
}

#[ignore]
#[test]
fn test() {
    let s = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    let want = 6;
    let got = part1(&generator(s));

    assert_eq!(want, got);
}

#[test]
fn test_input() {
    let s = std::fs::read_to_string("input/2018/day19.txt").unwrap();
    println!("{}", s);

    let want = 0;
    let got = part1(&generator(&s));

    assert_eq!(want, got);
}