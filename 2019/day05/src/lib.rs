use std::convert::TryFrom;

pub fn solve_part_1(memory: &mut [i32], input: i32) -> Result<i32, String> {
    let mut address = 0;
    let mut res = 0;

    loop {
        let op_code = OpCode::try_from(memory[address])?;

        match op_code {
            OpCode::Add(m1, m2, m3) => {
                let (p1, p2) = (
                    mem_get(m1, address + 1, memory),
                    mem_get(m2, address + 2, memory),
                );
                mem_set(m3, address + 3, memory, p1 + p2);

                address += 4;
            }
            OpCode::Mult(m1, m2, m3) => {
                let (p1, p2) = (
                    mem_get(m1, address + 1, memory),
                    mem_get(m2, address + 2, memory),
                );
                mem_set(m3, address + 3, memory, p1 * p2);

                address += 4;
            }
            OpCode::Save(m1) => {
                mem_set(m1, address + 1, memory, input);
                address += 2;
            }
            OpCode::Out(m1) => {
                res = mem_get(m1, address + 1, memory);
                address += 2;
            }
            OpCode::Jit(m1, m2) => {
                if mem_get(m1, address + 1, memory) != 0 {
                    address = mem_get(m2, address + 2, memory) as usize;
                } else {
                    address += 3;
                }
            }
            OpCode::Jif(m1, m2) => {
                if mem_get(m1, address + 1, memory) == 0 {
                    address = mem_get(m2, address + 2, memory) as usize;
                } else {
                    address += 3;
                }
            }
            OpCode::Lt(m1, m2, m3) => {
                let (p1, p2) = (
                    mem_get(m1, address + 1, memory),
                    mem_get(m2, address + 2, memory),
                );
                mem_set(m3, address + 3, memory, (p1 < p2) as i32);
                address += 4;
            }
            OpCode::Eq(m1, m2, m3) => {
                let (p1, p2) = (
                    mem_get(m1, address + 1, memory),
                    mem_get(m2, address + 2, memory),
                );
                mem_set(m3, address + 3, memory, (p1 == p2) as i32);

                address += 4;
            }
            OpCode::Quit => {
                return Ok(res);
            }
        }
    }
}

fn mem_set(mode: Mode, address: usize, memory: &mut [i32], v: i32) {
    match mode {
        Mode::Pos => memory[memory[address] as usize] = v,
        Mode::Imm => memory[address] = v,
    }
}

fn mem_get(mode: Mode, address: usize, memory: &[i32]) -> i32 {
    match mode {
        Mode::Imm => memory[address],
        Mode::Pos => memory[memory[address] as usize],
    }
}

enum Mode {
    Pos,
    Imm,
}

impl TryFrom<i32> for Mode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Imm),
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
    Eq(Mode, Mode, Mode),
}

impl TryFrom<i32> for OpCode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let de = value % 100;
        let c = value / 100 % 10;
        let b = value / 1000 % 10;
        let a = value / 10000 % 10;

        match de {
            1 => Ok(OpCode::Add(
                Mode::try_from(c)?,
                Mode::try_from(b)?,
                Mode::try_from(a)?,
            )),
            2 => Ok(OpCode::Mult(
                Mode::try_from(c)?,
                Mode::try_from(b)?,
                Mode::try_from(a)?,
            )),
            3 => Ok(OpCode::Save(Mode::try_from(c)?)),
            4 => Ok(OpCode::Out(Mode::try_from(c)?)),
            5 => Ok(OpCode::Jit(Mode::try_from(c)?, Mode::try_from(b)?)),
            6 => Ok(OpCode::Jif(Mode::try_from(c)?, Mode::try_from(b)?)),
            7 => Ok(OpCode::Lt(
                Mode::try_from(c)?,
                Mode::try_from(b)?,
                Mode::try_from(a)?,
            )),
            8 => Ok(OpCode::Eq(
                Mode::try_from(c)?,
                Mode::try_from(b)?,
                Mode::try_from(a)?,
            )),
            99 => Ok(OpCode::Quit),
            _ => Err(format!("OpCode not recognized: {}", de)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_short_one() {
        let mut memory: Vec<i32> = "3,9,8,9,10,9,4,9,99,-1,8"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 8).unwrap(), 1)
    }

    #[test]
    fn test_short_two() {
        let mut memory: Vec<i32> = "3,9,7,9,10,9,4,9,99,-1,8"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 4).unwrap(), 1)
    }

    #[test]
    fn test_short_three() {
        let mut memory: Vec<i32> = "3,3,1108,-1,8,3,4,3,99"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 8).unwrap(), 1)
    }

    #[test]
    fn test_short_four() {
        let mut memory: Vec<i32> = "3,3,1107,-1,8,3,4,3,99"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 7).unwrap(), 1);
    }

    #[test]
    fn test_short_five() {
        let mut memory: Vec<i32> = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 0).unwrap(), 0);
    }

    #[test]
    fn test_short_six() {
        let mut memory: Vec<i32> = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 0).unwrap(), 0);
    }

    #[test]
    fn test_complete() {
        let mut memory: Vec<i32> = fs::read_to_string("test.in")
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        assert_eq!(solve_part_1(&mut memory, 7).unwrap(), 999);
    }
}
