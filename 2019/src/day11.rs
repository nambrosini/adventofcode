use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day11, part1)]
pub fn part1(memory: &[i64]) -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    let mut pc = Computer::new(memory);
    let mut dir = Direction::N;
    let mut pos = (0, 0);

    loop {
        let v = if let Some(val) = map.get(&pos) {
            *val
        } else {
            0
        };

        if let Some(out) = pc.run(Some(v)) {
            let entry = map.entry(pos).or_insert(0);
            *entry = out;
        } else {
            break;
        }

        if let Some(out) = pc.run(Some(v)) {
            dir = dir.turn(out);
            pos = dir.move_forward(pos);
        }
    }

    map.len()
}

#[aoc(day11, part2)]
pub fn part2(memory: &[i64]) -> String {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    let mut pc = Computer::new(memory);
    let mut dir = Direction::N;
    let mut pos = (0, 0);

    let mut i = 0;

    loop {
        let v = if i == 0 { 
            1
        } else if let Some(val) = map.get(&pos) {
            *val
        } else {
            0
        };

        if let Some(out) = pc.run(Some(v)) {
            let entry = map.entry(pos).or_insert(0);
            *entry = out;
        } else {
            break;
        }
        println!("{}", v);
        if let Some(out) = pc.run(Some(v)) {
            dir = dir.turn(out);
            pos = dir.move_forward(pos);
        }

        i += 1;
    }

    let min_x = map.keys().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = map.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;

    let mut s = String::from("\n");

    for x in min_x..max_x {
        for y in min_y..max_y {
            if let Some(v) = map.get(&(x, y)) {
                if v == &1 {
                    s.push('#');
                }
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }

    s
}

enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    fn turn(&self, dir: i64) -> Self {
        match dir {
            0 => {
                match self {
                    Self::N => Self::W,
                    Self::E => Self::N,
                    Self::S => Self::E,
                    Self::W => Self::S
                }
            },
            1 => {
                match self {
                    Self::N => Self::E,
                    Self::E => Self::S,
                    Self::S => Self::W,
                    Self::W => Self::N
                }
            },
            _ => unreachable!()
        }
    }

    fn move_forward(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Self::N => (x - 1, y),
            Self::E => (x, y + 1), 
            Self::S => (x + 1, y),
            Self::W => (x, y - 1)
        }
    }
}


#[derive(Debug, Clone)]
struct Computer {
    memory: HashMap<i64, i64>,
    position: i64
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        let mut map = HashMap::new();

        for (i, e) in memory.iter().enumerate() {
            map.insert(i as i64, *e);
        }

        Self { 
            memory: map, 
            position: 0 
        }
    }

    fn run(&mut self, input: Option<i64>) -> Option<i64> {
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
                Operation::Save => {
                    if let Some(inp) = input {
                        self.set_mem(1, inp, Mode::Position);
                    }
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
                Operation::Exit => {
                    return None;
                },
            }
        }
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