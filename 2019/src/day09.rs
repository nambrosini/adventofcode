use std::collections::HashMap;

#[aoc_generator(day09)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day09, part1)]
pub fn part1(mem: &[i64]) -> i64 {
    Intcode::new(mem).run(1)
}

#[aoc(day09, part2)]
pub fn part2(mem: &[i64]) -> i64 {
    Intcode::new(mem).run(2)
}

pub struct Intcode {
    mem: HashMap<usize, i64>,
    pos: usize,
    rel_base: usize,
}

impl Intcode {
    fn new(mem: &[i64]) -> Self {
        Self {
            mem: mem.iter().copied().enumerate().collect(),
            pos: 0,
            rel_base: 0,
        }
    }

    fn run(&mut self, input: i64) -> i64 {
        let mut res = 0;
        loop {
            let op: Opcode = self.mem[&self.pos].into();
            match op {
                Opcode::Add(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);

                    self.set_mem(3, v1 + v2, m3);
                    self.pos += 4;
                }
                Opcode::Mul(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);

                    self.set_mem(3, v1 * v2, m3);
                    self.pos += 4;
                }
                Opcode::Save(m1) => {
                    self.set_mem(1, input, m1);
                    self.pos += 2;
                }
                Opcode::Out(m1) => {
                    res = self.get_mem(1, m1);
                    println!("{}", res);
                    self.pos += 2;
                }
                Opcode::Jit(m1, m2) => {
                    if self.get_mem(1, m1) != 0 {
                        self.pos = self.get_mem(2, m2) as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                Opcode::Jif(m1, m2) => {
                    if self.get_mem(1, m1) == 0 {
                        self.pos = self.get_mem(2, m2) as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                Opcode::Lt(m1, m2, m3) => {
                    let v = i64::from(self.get_mem(1, m1) < self.get_mem(2, m2));
                    self.set_mem(3, v, m3);
                    self.pos += 4;
                }
                Opcode::Eq(m1, m2, m3) => {
                    let v = i64::from(self.get_mem(1, m1) == self.get_mem(2, m2));
                    self.set_mem(3, v, m3);
                    self.pos += 4;
                }
                Opcode::Rb(m1) => {
                    self.rel_base += self.get_mem(1, m1) as usize;
                    self.pos += 2;
                }
                Opcode::Exit => return res,
            }
        }
    }

    fn set_mem(&mut self, offset: usize, val: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        let entry = self.mem.entry(index).or_insert(0);
        *entry = val;
    }

    fn get_mem(&self, offset: usize, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        self.mem[&index]
    }

    fn get_index(&self, offset: usize, mode: Mode) -> usize {
        match mode {
            Mode::Pos => self.mem[&(self.pos + offset)] as usize,
            Mode::Imm => self.pos + offset,
            Mode::Rel => self.mem[&(self.pos + offset)] as usize + self.rel_base,
        }
    }
}

#[derive(Debug)]
enum Mode {
    Pos,
    Imm,
    Rel,
}

impl From<i64> for Mode {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Pos,
            1 => Self::Imm,
            2 => Self::Rel,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Rb(Mode),
    Exit,
}

impl From<i64> for Opcode {
    fn from(x: i64) -> Self {
        let op = x % 100;
        let m1: Mode = (x / 100 % 10).into();
        let m2: Mode = (x / 1000 % 10).into();
        let m3: Mode = (x / 10000 % 10).into();

        match op {
            1 => Self::Add(m1, m2, m3),
            2 => Self::Mul(m1, m2, m3),
            3 => Self::Save(m1),
            4 => Self::Out(m1),
            5 => Self::Jit(m1, m2),
            6 => Self::Jif(m1, m2),
            7 => Self::Lt(m1, m2, m3),
            8 => Self::Eq(m1, m2, m3),
            9 => Self::Rb(m1),
            99 => Self::Exit,
            _ => unreachable!(),
        }
    }
}
