use itertools::Itertools;

#[aoc_generator(day05)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day05, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut pc = Computer::new(input);

    let mut last = 0;

    while let Some(v) = pc.run(1) {
        last = v;
    }

    last
}

#[aoc(day05, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut pc = Computer::new(input);

    pc.run(5).unwrap()
}

struct Computer {
    memory: Vec<i64>,
    position: usize
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        Self { 
            memory: memory.to_vec(), 
            position: 0 
        }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        loop {
            let op: Operation = self.memory[self.position].into();
            match op {
                Operation::Add(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);
                    self.set_mem(3, v1 + v2, m3);
                    self.position += 4;
                },
                Operation::Mul(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);
                    self.set_mem(3, v1 * v2, m3);
                    self.position += 4;
                },
                Operation::Save => {
                    self.set_mem(1, input, Mode::Position);
                    self.position += 2;
                },
                Operation::Out(m1) => {
                    let output = self.get_mem(1, m1);
                    self.position += 2;
                    return Some(output);
                },
                Operation::Jit(m1, m2) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 != 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                },
                Operation::Jif(m1, m2) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 == 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                },
                Operation::Lt(m1, m2, m3) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 < p2 {
                        self.set_mem(3, 1, m3);
                    } else {
                        self.set_mem(3, 0, m3);
                    }
                    self.position += 4;
                },
                Operation::Eq(m1, m2, m3) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 == p2 {
                        self.set_mem(3, 1, m3);
                    } else {
                        self.set_mem(3, 0, m3);
                    }
                    self.position += 4;
                },
                Operation::Exit => {
                    return None;
                },
            }
        }
    }

    fn get_mem(&self, offset: usize, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        self.memory[index]
    }

    fn set_mem(&mut self, offset: usize, value: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        self.memory[index] = value
    }

    fn get_index(&self, offset: usize, mode: Mode) -> usize {
        match mode {
            Mode::Position => self.memory[self.position + offset] as usize,
            Mode::Immediate => self.position + offset
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save,
    Out(Mode),
    Exit,
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode)
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate
}

impl From<i64> for Operation {
    fn from(i: i64) -> Self {
        let code = i % 100;
        let m1 = (i / 100) % 10;
        let m2 = (i / 1000) % 10;
        let m3 = (i / 10000) % 10;

        match code {
            1 => Self::Add(m1.into(), m2.into(), m3.into()),
            2 => Self::Mul(m1.into(), m2.into(), m3.into()),
            3 => Self::Save,
            4 => Self::Out(m1.into()),
            5 => Self::Jit(m1.into(), m2.into()),
            6 => Self::Jif(m1.into(), m2.into()),
            7 => Self::Lt(m1.into(), m2.into(), m3.into()),
            8 => Self::Eq(m1.into(), m2.into(), m3.into()),
            99 => Self::Exit,
            _ => unreachable!()
        }
    }
}

impl From<i64> for Mode {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => unreachable!()
        }
    }
}