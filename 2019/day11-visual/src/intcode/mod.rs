pub mod color;
mod direction;
mod mode;
mod opcode;

use color::Color;
use direction::Direction;
use mode::Mode;
use opcode::OpCode;
use std::convert::TryInto;

use std::collections::HashMap;

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
            Mode::Pos => self
                .memory
                .entry(self.address + address_offset)
                .or_insert(0)
                .clone(),
            Mode::Rel => {
                self.memory
                    .entry(self.address + address_offset)
                    .or_insert(0)
                    .clone()
                    + self.relative_base
            }
        };

        *self.memory.entry(addr).or_insert(0)
    }

    fn mem_set(&mut self, mode: Mode, address_offset: i64, v: i64) {
        let addr = match mode {
            Mode::Pos => *self
                .memory
                .entry(self.address + address_offset)
                .or_insert(0),
            Mode::Imm => self.address + address_offset,
            Mode::Rel => {
                *self
                    .memory
                    .entry(self.address + address_offset)
                    .or_insert(0)
                    + self.relative_base
            }
        };

        self.memory.insert(addr, v);
    }
}
