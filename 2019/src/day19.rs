use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day19, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut count = 0;

    for i in 0..50 {
        let mut was_one = false;
        for j in 0..50 {
            let mut pc = Computer::new(input);

            if pc.run(&[i, j]) == 1 {
                was_one = true;
                count += 1;
            } else if was_one {
                break;
            }
        }
    }

    count
}

#[aoc(day19, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut pc = Computer::new(input);
    let mut map = HashMap::new();

    for i in 500..10000 {
        let mut was_one = false;
        for j in i..10000 {
            let current = pc.run(&[i, j]);
            map.insert((i, j), current);

            if current == 1 {
                was_one = true;
                let mut found = true;
                for k in 1..100 {
                    let x = if let Some(x) = map.get(&(i + k, j)) {
                        *x
                    } else {
                        let x = pc.run(&[i + k, j]);
                        map.insert((i + k, j), x);
                        x
                    };

                    let y = if let Some(y) = map.get(&(i, j + k)) {
                        *y
                    } else {
                        let y = pc.run(&[i, j + k]);
                        map.insert((i, j + k), y);
                        y
                    };

                    if x == 0 || y == 0 {
                        found = false;
                        break;
                    }
                }
                if found {
                    return i * 10000 + j;
                }
            } else if was_one {
                break;
            }
        }
    }

    unreachable!()
}

struct Computer {
    start_memory: HashMap<i64, i64>,
    memory: HashMap<i64, i64>,
    position: i64,
    relative_base: i64
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        let mut map = HashMap::new();

        for (i, e) in memory.iter().enumerate() {
            map.insert(i as i64, *e);
        }

        Self { 
            start_memory: map.clone(),
            memory: map, 
            position: 0,
            relative_base: 0
        }
    }

    fn run(&mut self, input: &[i64]) -> i64 {
        self.reset();
        let mut input = input.to_vec();
        loop {
            let op: Operation = self.memory[&self.position].into();
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
                Operation::Save(m1) => {
                    if !input.is_empty() {
                        let v = input.remove(0);
                        self.set_mem(1, v, m1);
                    }
                    self.position += 2;
                },
                Operation::Out(m1) => {
                    let output = self.get_mem(1, m1);
                    self.position += 2;
                    return output;
                },
                Operation::Jit(m1, m2) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 != 0 {
                        self.position = p2;
                    } else {
                        self.position += 3;
                    }
                },
                Operation::Jif(m1, m2) => {
                    let p1 = self.get_mem(1, m1);
                    let p2 = self.get_mem(2, m2);

                    if p1 == 0 {
                        self.position = p2;
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
                Operation::Rb(m1) => {
                    let p1 = self.get_mem(1, m1);
                    self.relative_base += p1;
                    self.position += 2;
                }
                Operation::Exit => {
                    return 0;
                },
            }
        }
    }

    fn reset(&mut self) {
        self.position = 0;
        self.relative_base = 0;
        self.memory = self.start_memory.clone();
    }

    fn get_mem(&mut self, offset: i64, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        let entry = self.memory.entry(index).or_insert(0);
        *entry
    }

    fn set_mem(&mut self, offset: i64, value: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        let entry = self.memory.entry(index).or_insert(0);
        *entry = value
    }

    fn get_index(&self, offset: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Position => self.memory[&(self.position + offset)],
            Mode::Immediate => self.position + offset,
            Mode::Relative => self.memory[&(self.position + offset)] + self.relative_base
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Exit,
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Rb(Mode)
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative
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
            3 => Self::Save(m1.into()),
            4 => Self::Out(m1.into()),
            5 => Self::Jit(m1.into(), m2.into()),
            6 => Self::Jif(m1.into(), m2.into()),
            7 => Self::Lt(m1.into(), m2.into(), m3.into()),
            8 => Self::Eq(m1.into(), m2.into(), m3.into()),
            9 => Self::Rb(m1.into()),
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
            2 => Self::Relative,
            _ => unreachable!()
        }
    }
}