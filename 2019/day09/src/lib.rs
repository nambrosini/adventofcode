use std::convert::{TryFrom, TryInto};

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

    pub fn run(&mut self, input: i64) -> Result<i64, String> {
        let mut res = 0;
        loop {
            let op_code: OpCode = self.memory[self.address].try_into()?;
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
                    self.mem_set(m1, 1, input);
                    self.address += 2;
                }
                OpCode::Out(m1) => {
                    res = self.mem_get(m1, 1);
                    self.address += 2;
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
                    return Ok(res);
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
