use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::collections::HashMap;

type Pos = (i64, i64);
type Tile = (TileId, Pos);

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(memory: &[i64]) -> usize {
    let mut arcade = Arcade::new(memory);
    let mut map = HashMap::new();

    while let Some(tile) = arcade.get_tile() {
        map.insert(tile.1, tile.0);
    }

    map.values().filter(|&&t| t == TileId::Block).count()
}

#[aoc(day13, part2)]
pub fn part2(memory: &[i64]) -> i64 {
    let mut map = HashMap::new();
    let mut arcade = Arcade::new(memory);

    arcade.set_free_play();

    let mut score = 0;
    let mut input = 0;

    while let Some(res) = arcade.get_res(input) {
        match res {
            Res::Score(s) => score = s,
            Res::PosTile(t) => {
                map.insert(t.1, t.0);
            }
        }

        let paddle_x = map.iter()
            .find(|(_, &t)| t == TileId::HPaddle)
            .map(|(p, _)| p)
            .unwrap_or(&(0, 0))
            .0;

        let ball_x = map.iter()
            .find(|(_, &t)| t == TileId::Ball)
            .map(|(p, _)| p)
            .unwrap_or(&(0, 0))
            .0;

        input = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0
        };
    }

    score
}

pub struct IntCode {
    memory: Vec<i64>,
    address: usize,
    relative_base: i64,
}

impl IntCode {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            address: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self, mut input: Option<i64>) -> Option<i64> {
        loop {
            let op_code: OpCode = self.memory[self.address].try_into().unwrap();
            let mut v = 0;

            if self.address >= self.memory.len() {
                v = self.address;
            } else if (self.memory[self.address] as usize) >= self.memory.len() {
                v = self.memory[self.address] as usize;
            } else if ((self.memory[self.address] + self.relative_base) as usize)
                >= self.memory.len()
            {
                v = (self.memory[self.address] + self.relative_base) as usize;
            }

            if v > 0 {
                let mut new_slice: Vec<i64> = (0..v - self.memory.len()).map(|_| 0i64).collect();
                self.memory.append(&mut new_slice);
            }

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
                    self.mem_set(m1, 1, input.take().unwrap());
                    self.address += 2;
                }
                OpCode::Out(m1) => {
                    let p1 = self.mem_get(m1, 1);
                    self.address += 2;
                    return Some(p1);
                }
                OpCode::Jit(m1, m2) => {
                    if self.mem_get(m1, 1) != 0 {
                        self.address = self.mem_get(m2, 2) as usize;
                    } else {
                        self.address += 3;
                    }
                }
                OpCode::Jif(m1, m2) => {
                    if self.mem_get(m1, 1) == 0 {
                        self.address = self.mem_get(m2, 2) as usize;
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
                    return None;
                }
            }
        }
    }

    fn mem_get(&self, mode: Mode, address_offset: usize) -> i64 {
        match mode {
            Mode::Imm => self.memory[self.address + address_offset],
            Mode::Pos => self.memory[self.memory[self.address + address_offset] as usize],
            Mode::Rel => {
                self.memory
                    [(self.memory[self.address + address_offset] + self.relative_base) as usize]
            }
        }
    }

    fn mem_set(&mut self, mode: Mode, address_offset: usize, v: i64) {
        let addr = match mode {
            Mode::Pos => self.memory[self.address + address_offset] as usize,
            Mode::Imm => self.address + address_offset,
            Mode::Rel => (self.memory[self.address + address_offset] + self.relative_base) as usize,
        };

        self.memory[addr] = v;
    }
}

enum Mode {
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

enum OpCode {
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

struct Arcade {
    int_code: IntCode
}

impl Arcade {
    fn new(memory: &[i64]) -> Self {
        Self {
            int_code: IntCode::new(memory.to_vec())
        }
    }

    fn get_tile(&mut self) -> Option<Tile> {
        let x = self.int_code.run(None)?;
        let y = self.int_code.run(None)?;
        let t = self.int_code.run(None)?.try_into().unwrap();

        Some((t, (x, y)))
    }

    fn get_res(&mut self, input: i64) -> Option<Res> {
        let x = self.int_code.run(Some(input))?;
        let y = self.int_code.run(None)?;
        let t = self.int_code.run(None).unwrap();

        if x == -1 && y == 0 {
            Some(Res::Score(t)) 
        } else {
            Some(Res::PosTile((t.try_into().unwrap(), (x, y))))
        }
    }

    fn set_free_play(&mut self) {
        self.int_code.memory[0] = 2;
    }
}

#[derive(PartialEq, Copy, Clone)]
enum TileId {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball
}

impl TryFrom<i64> for TileId {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::HPaddle),
            4 => Ok(Self::Ball),
            _ => Err(format!("Unknown value: {}", value))
        }
    }
}

enum Res {
    PosTile(Tile),
    Score(i64)
}