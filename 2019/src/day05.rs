#[aoc_generator(day05)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day05, part1)]
pub fn part1(mem: &[i64]) -> i64 {
    let mut pc = Intcode::new(mem);

    let mut last = 0;

    while let Some(v) = pc.run(1) {
        last = v;
    }

    last
}

#[aoc(day05, part2)]
pub fn part2(mem: &[i64]) -> i64 {
    let mut pc = Intcode::new(mem);

    let mut last = 0;

    while let Some(v) = pc.run(5) {
        last = v;
    }

    last
}

pub struct Intcode {
    mem: Vec<i64>,
    pos: usize,
}

impl Intcode {
    fn new(mem: &[i64]) -> Self {
        Self {
            mem: mem.to_vec(),
            pos: 0,
        }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        loop {
            let op: Opcode = self.mem[self.pos].into();
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
                Opcode::Save => {
                    self.set_mem(1, input, Mode::Pos);
                    self.pos += 2;
                }
                Opcode::Out(m1) => {
                    let out = self.get_mem(1, m1);
                    self.pos += 2;
                    return Some(out);
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
                Opcode::Exit => return None,
            }
        }
    }

    fn set_mem(&mut self, offset: usize, val: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        self.mem[index] = val;
    }

    fn get_mem(&self, offset: usize, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        self.mem[index]
    }

    fn get_index(&self, offset: usize, mode: Mode) -> usize {
        match mode {
            Mode::Pos => self.mem[self.pos + offset] as usize,
            Mode::Imm => self.pos + offset,
        }
    }
}

enum Mode {
    Pos,
    Imm,
}

impl From<i64> for Mode {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Pos,
            1 => Self::Imm,
            _ => unreachable!(),
        }
    }
}

enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save,
    Out(Mode),
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
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
            3 => Self::Save,
            4 => Self::Out(m1),
            5 => Self::Jit(m1, m2),
            6 => Self::Jif(m1, m2),
            7 => Self::Lt(m1, m2, m3),
            8 => Self::Eq(m1, m2, m3),
            99 => Self::Exit,
            _ => unreachable!(),
        }
    }
}