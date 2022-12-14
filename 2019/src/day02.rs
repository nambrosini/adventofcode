#[aoc_generator(day02)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day02, part1)]
pub fn part1(mem: &[i64]) -> i64 {
    let mut pc = Intcode::new(mem);
    pc.run(12, 2)
}

#[aoc(day02, part2)]
pub fn part2(mem: &[i64]) -> i64 {
    let mut pc = Intcode::new(mem);

    for i in 0..99 {
        for j in 0..99 {
            let res = pc.run(i, j);

            if res == 19690720 {
                return 100 * i + j;
            }

            pc.reset(mem);
        }
    }

    unreachable!()
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

    fn run(&mut self, a: i64, b: i64) -> i64 {
        self.mem[1] = a;
        self.mem[2] = b;
        loop {
            let op: Opcode = self.mem[self.pos].into();

            match op {
                Opcode::Add => {
                    let v1 = self.get_mem(1);
                    let v2 = self.get_mem(2);

                    self.set_mem(3, v1 + v2);
                }
                Opcode::Mul => {
                    let v1 = self.get_mem(1);
                    let v2 = self.get_mem(2);

                    self.set_mem(3, v1 * v2);
                }
                Opcode::Exit => return self.mem[0],
            }
            self.pos += 4;
        }
    }

    fn set_mem(&mut self, offset: usize, val: i64) {
        let index = self.mem[self.pos + offset] as usize;
        self.mem[index] = val;
    }

    fn get_mem(&self, offset: usize) -> i64 {
        self.mem[self.mem[self.pos + offset] as usize]
    }

    fn reset(&mut self, mem: &[i64]) {
        self.mem = mem.to_vec();
        self.pos = 0;
    }
}

enum Opcode {
    Add,
    Mul,
    Exit,
}

impl From<i64> for Opcode {
    fn from(x: i64) -> Self {
        match x {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Exit,
            _ => unreachable!(),
        }
    }
}
