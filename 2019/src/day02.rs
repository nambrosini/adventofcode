use itertools::Itertools;

#[aoc_generator(day02)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day02, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut pc = Computer::new(&input);

    pc.run(12, 2)
}

#[aoc(day02, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut pc = Computer::new(input);

    for i in 0..100 {
        for j in 0..100 {
            if pc.run(i, j) == 19690720 {
                return 100 * i + j
            };
            pc.reset(input);
        }
    }

    unreachable!()
}

struct Computer {
    memory: Vec<i64>,
    position: usize
}

impl Computer {
    fn new(memory: &[i64]) -> Computer {
        Self { memory: memory.to_vec(), position: 0 }
    }

    fn run(&mut self, a: i64, b: i64) -> i64 {
        self.memory[1] = a;
        self.memory[2] = b;
        loop {
            match self.memory[self.position] {
                1 => {
                    let v1 = self.get_mem(1);
                    let v2 = self.get_mem(2);
                    self.set_mem(3, v1 + v2);
                },
                2 => {
                    let v1 = self.get_mem(1);
                    let v2 = self.get_mem(2);
                    self.set_mem(3, v1 * v2);
                },
                99 => {
                    return self.memory[0];
                },
                _ => unreachable!()
            }
            self.position += 4
        }
    }

    fn reset(&mut self, memory: &[i64]) {
        self.memory = memory.to_vec();
        self.position = 0;
    }

    fn get_mem(&self, offset: usize) -> i64 {
        self.memory[self.memory[self.position + offset] as usize]
    }

    fn set_mem(&mut self, offset: usize, value: i64) {
        let index: usize = self.memory[self.position + offset] as usize;
        self.memory[index] = value
    }
}