use itertools::Itertools;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> usize {
    let mut cpu = Cpu::new(input);

    cpu.run()
}

#[aoc(day5, part2)]
pub fn part2(input: &[i32]) -> usize {
    let mut cpu = Cpu::new(input);

    cpu.run_part2()
}

struct Cpu {
    memory: Vec<i32>,
}

impl Cpu {
    fn new(memory: &[i32]) -> Self {
        Self {
            memory: memory.to_vec(),
        }
    }

    fn run(&mut self) -> usize {
        let mut steps = 0;
        let mut address = 0;

        while address < self.memory.len() {
            let new_address = address + self.memory[address] as usize;
            self.memory[address] += 1;
            address = new_address;

            steps += 1;
        }

        steps
    }

    fn run_part2(&mut self) -> usize {
        let mut steps = 0;
        let mut address = 0;

        while address < self.memory.len() {
            let new_address = address + self.memory[address] as usize;
            if self.memory[address] >= 3 {
                self.memory[address] -= 1;
            } else {
                self.memory[address] += 1;
            }
            address = new_address;

            steps += 1;
        }

        steps
    }
}
