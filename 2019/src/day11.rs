use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> HashMap<i64, i64> {
    let input = input.split(',').map(|x| x.parse().unwrap()).collect_vec();

    let mut memory: HashMap<i64, i64> = HashMap::new();

    for (i, &e) in input.iter().enumerate() {
        memory.insert(i as i64, e);
    }

    memory
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<i64, i64>) -> usize {
    let memory = input.clone();
    IntCode::new(memory).run(0).unwrap().len() - 1
}

#[aoc(day11, part2)]
pub fn part2(input: &HashMap<i64, i64>) -> usize {
    let memory = input.clone();
    let result = IntCode::new(memory).run(1).unwrap();

    let min_x = result.keys().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = result.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = result.keys().max_by_key(|(x, _)| x).unwrap().0 - min_x + 1;
    let max_y = result.keys().max_by_key(|(_, y)| y).unwrap().1 - min_y + 1;

    let mut grid: Vec<Vec<Color>> = vec![vec![Color::Black; max_x as usize]; max_y as usize];

    for ((x, y), c) in &result {
        grid[(y - min_y) as usize][(x - min_x) as usize] = c.clone();
    }

    println!("Part 2:");

    for i in grid {
        for j in i {
            match j {
                Color::Black => print!(" "),
                Color::White => print!("▓")
            }
        }
        println!();
    }

    0
}

type Ship = HashMap<(i64, i64), Color>;

pub struct IntCode {
    memory: HashMap<i64, i64>,
    address: i64,
    relative_base: i64,
}

impl IntCode {
    pub fn new(memory: HashMap<i64, i64>) -> Self {
        Self {
            memory,
            address: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self, input: i64) -> Result<HashMap<(i64, i64), Color>, String> {
        let mut res = (input, 0);
        let mut first = true;
        let mut coord: (i64, i64) = (0, 0);
        let mut ship: Ship = Ship::new();
        let mut direction = Direction::Up;

        loop {
            let op_code: OpCode = self.memory[&self.address].try_into()?;

            match op_code {
                OpCode::Add(m1, m2, m3) => {
                    let (p1, p2) = (self.mem_get(m1, 1), self.mem_get(m2, 2));
                    self.mem_set(m3, 3, p1 + p2);

                    self.address += 4;
                }
                OpCode::Mult(m1, m2, m3) => {
                    let (p1, p2) = (self.mem_get(m1, 1), self.mem_get(m2, 2));
                    self.mem_set(m3, 3, p1 * p2);

                    self.address += 4;
                }
                OpCode::Save(m1) => {
                    self.mem_set(m1, 1, res.0);
                    self.address += 2;
                }
                OpCode::Out(m1) => {
                    if first {
                        res.0 = self.mem_get(m1, 1);
                    } else {
                        res.1 = self.mem_get(m1, 1);
                        ship.insert(coord, res.0.try_into()?);
                        direction = Direction::turn(&direction, res.1);
                        let m = Direction::get_direction_param(&direction);
                        coord = (coord.0 + m.0, coord.1 + m.1);
                        res.0 = ship.entry(coord).or_insert(Color::Black).clone().into();
                    }
                    first = !first;
                    self.address += 2;
                }
                OpCode::Jit(m1, m2) => {
                    if self.mem_get(m1, 1) != 0 {
                        self.address = self.mem_get(m2, 2);
                    } else {
                        self.address += 3;
                    }
                }
                OpCode::Jif(m1, m2) => {
                    if self.mem_get(m1, 1) == 0 {
                        self.address = self.mem_get(m2, 2);
                    } else {
                        self.address += 3;
                    }
                }
                OpCode::Lt(m1, m2, m3) => {
                    let (p1, p2) = (self.mem_get(m1, 1), self.mem_get(m2, 2));
                    self.mem_set(m3, 3, (p1 < p2) as i64);
                    self.address += 4;
                }
                OpCode::Eq(m1, m2, m3) => {
                    let (p1, p2) = (self.mem_get(m1, 1), self.mem_get(m2, 2));
                    self.mem_set(m3, 3, (p1 == p2) as i64);

                    self.address += 4;
                }
                OpCode::Arb(m1) => {
                    let p1 = self.mem_get(m1, 1);
                    self.relative_base += p1;
                    self.address += 2;
                }
                OpCode::Quit => {
                    return Ok(ship);
                }
            }
        }
    }

    fn mem_get(&mut self, mode: Mode, address_offset: i64) -> i64 {
        let addr = match mode {
            Mode::Imm => self.address + address_offset,
            Mode::Pos => *self.memory.entry(self.address + address_offset).or_insert(0),
            Mode::Rel => {
                *self.memory.entry(self.address + address_offset).or_insert(0) + self.relative_base
            }
        };

        *self.memory.entry(addr).or_insert(0)
    }

    fn mem_set(&mut self, mode: Mode, address_offset: i64, v: i64) {
        let addr = match mode {
            Mode::Pos => *self.memory.entry(self.address + address_offset).or_insert(0),
            Mode::Imm => self.address + address_offset,
            Mode::Rel => *self.memory.entry(self.address + address_offset).or_insert(0) + self.relative_base,
        };

        self.memory.insert(addr, v);
    }
}

#[derive(Clone)]
pub enum Color {
    Black,
    White
}

impl From<Color> for i64 {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(format!("Color not recognized: {}", value))
        }
    }
}


pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn turn(direction: &Self, dir: i64) -> Self{
        match direction {
            Direction::Up => {
                if dir == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            Direction::Left => {
                if dir == 0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            },
            Direction::Down => {
                if dir == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            },
            Direction::Right => {
                if dir == 0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }

    pub fn get_direction_param(direction: &Self) -> (i64, i64) {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}

pub enum Mode {
    Pos,
    Imm,
    Rel,
}

impl TryFrom<i64> for Mode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Imm),
            2 => Ok(Mode::Rel),
            _ => Err(format!("Unknown mode: {}", value)),
        }
    }
}

pub enum OpCode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Quit,
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Arb(Mode),
    Eq(Mode, Mode, Mode),
}

impl TryFrom<i64> for OpCode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let de = value % 100;
        let c = value / 100 % 10;
        let b = value / 1000 % 10;
        let a = value / 10000 % 10;

        match de {
            1 => Ok(OpCode::Add(c.try_into()?, b.try_into()?, a.try_into()?)),
            2 => Ok(OpCode::Mult(c.try_into()?, b.try_into()?, a.try_into()?)),
            3 => Ok(OpCode::Save(c.try_into()?)),
            4 => Ok(OpCode::Out(c.try_into()?)),
            5 => Ok(OpCode::Jit(c.try_into()?, b.try_into()?)),
            6 => Ok(OpCode::Jif(c.try_into()?, b.try_into()?)),
            7 => Ok(OpCode::Lt(c.try_into()?, b.try_into()?, a.try_into()?)),
            8 => Ok(OpCode::Eq(c.try_into()?, b.try_into()?, a.try_into()?)),
            9 => Ok(OpCode::Arb(c.try_into()?)),
            99 => Ok(OpCode::Quit),
            _ => Err(format!("OpCode not recognized: {}", de)),
        }
    }
}