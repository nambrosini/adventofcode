use itertools::Itertools;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .split('\t')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec()
}

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut debugger = Debugger::new(input);

    debugger.run().len() - 1
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut debugger = Debugger::new(input);

    let v = debugger.run();

    let last = v.last().unwrap();

    let mut index = 0;

    while index < v.len() {
        if v[index] == *last {
            break;
        }
        index += 1;
    }

    v.len() - index - 1
}

struct Debugger {
    memory: Vec<usize>,
}

impl Debugger {
    fn new(memory: &[usize]) -> Self {
        Self {
            memory: memory.to_vec(),
        }
    }

    fn step(&mut self) {
        let max = get_max_index(&self.memory);
        let val = max.1;

        let mut address = (max.0 + 1) % self.memory.len();
        self.memory[max.0] = 0;

        for _ in 0..val {
            self.memory[address] += 1;
            address = (address + 1) % self.memory.len();
        }
    }

    fn run(&mut self) -> Vec<String> {
        let mut v = vec![self.memory.iter().map(|x| x.to_string()).join(" ")];

        loop {
            self.step();

            let joined = self.memory.iter().map(|x| x.to_string()).join(" ");

            if v.contains(&joined) {
                break;
            }

            v.push(joined);
        }

        v
    }
}

fn get_max_index(memory: &[usize]) -> (usize, usize) {
    let mut max = memory[0];
    let mut index = 0;

    for (i, e) in memory.iter().enumerate().skip(1) {
        if *e > max {
            max = *e;
            index = i;
        }
    }

    (index, max)
}
