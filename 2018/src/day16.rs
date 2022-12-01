use std::collections::{HashMap, HashSet};

type Check = (Vec<usize>, Vec<usize>, Vec<usize>);

#[aoc_generator(day16)]
pub fn generator(input: &str) -> (Vec<Check>, Vec<Vec<usize>>) {
    let split: Vec<&str> = input.split("\n\n\n\n").collect();
    let first = split[0].split("\n\n")
        .map(|ins| {
            let vec: Vec<&str> = ins.lines().collect();
            let before: Vec<usize> = vec[0][9..19].split(", ").map(|x| x.parse().unwrap()).collect();
            let instruction: Vec<usize> = vec[1].split(' ').map(|x| x.parse().unwrap()).collect();
            let after: Vec<usize> = vec[2][9..19].split(", ").map(|x| x.parse().unwrap()).collect();
            (instruction, before, after)
        })
        .collect();
    let second = split[1].lines()
        .map(|ins| ins.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();
    
    (first, second)
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Check>, Vec<Vec<usize>>)) -> usize {
    input.0.iter()
        .filter(|x| get_matching(&x.0, &x.1, &x.2).len() >= 3)
        .count()
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Check>, Vec<Vec<usize>>)) -> usize {
    let mut map: HashMap<usize, HashSet<OpCode>> = HashMap::new();
    for (instruction, before, after) in input.0.iter() {
        let ops = get_matching(instruction, before, after);
        let entry = map.entry(instruction[0]).or_insert_with(HashSet::new);
        if entry.is_empty() {
            *entry = ops;
        } else {
            *entry = entry.intersection(&ops).copied().collect();
        }
    }

    let mut codes: HashMap<usize, OpCode> = HashMap::new();

    while !map.is_empty() {
        let mut new_map = map.clone();
        let singles = map.iter().filter(|(v, k)| k.len() == 1);

        for single in singles {
            let op = *single.1.iter().next().unwrap();
            codes.insert(*single.0, op);
            new_map.remove(single.0);

            for (_, v) in new_map.iter_mut() {
                if v.contains(&op) {
                    v.remove(&op);
                }
            }
        }
        map = new_map;
    }

    println!("{:#?}", codes);

    let mut device = Device { regs: vec![0; 4] };

    for instruction in &input.1 {
        device.exec_instruction(instruction, codes[&instruction[0]]);
    }

    device.regs[0]
}

struct Device {
    regs: Vec<usize>
}

impl Device {
    fn exec_instruction(&mut self, instruction: &[usize], op_code: OpCode) {
        match op_code {
            OpCode::Addr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = a + b;
            },
            OpCode::Addi => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = a + b;
            },
            OpCode::Mulr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = a * b;
            },
            OpCode::Muli => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = a * b;
            },
            OpCode::Banr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = a & b;
            },
            OpCode::Bani => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = a & b;
            },
            OpCode::Borr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = a | b;
            },
            OpCode::Bori => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = a | b;
            },
            OpCode::Setr => {
                let a = self.regs[instruction[1]];
                self.regs[instruction[3]] = a;
            },
            OpCode::Seti => {
                let a = instruction[1];
                self.regs[instruction[3]] = a;
            },
            OpCode::Gtir => {
                let a = instruction[1];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = (a > b) as usize;
            },
            OpCode::Gtri => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = (a > b) as usize;
            },
            OpCode::Gtrr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = (a > b) as usize;
            },
            OpCode::Eqir => {
                let a = instruction[1];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = (a == b) as usize;
            },
            OpCode::Eqri => {
                let a = self.regs[instruction[1]];
                let b = instruction[2];
                self.regs[instruction[3]] = (a == b) as usize;
            },
            OpCode::Eqrr => {
                let a = self.regs[instruction[1]];
                let b = self.regs[instruction[2]];
                self.regs[instruction[3]] = (a == b) as usize;
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OpCode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

impl OpCode {
    fn execute(&self, instruction: &Vec<usize>, before: &Vec<usize>, after: &Vec<usize>) -> bool {
        let mut res = before.clone();

        match self {
            OpCode::Addr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = a + b;
            },
            OpCode::Addi => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = a + b;
            },
            OpCode::Mulr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = a * b;
            },
            OpCode::Muli => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = a * b;
            },
            OpCode::Banr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = a & b;
            },
            OpCode::Bani => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = a & b;
            },
            OpCode::Borr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = a | b;
            },
            OpCode::Bori => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = a | b;
            },
            OpCode::Setr => {
                let a = before[instruction[1]];
                res[instruction[3]] = a;
            },
            OpCode::Seti => {
                let a = instruction[1];
                res[instruction[3]] = a;
            },
            OpCode::Gtir => {
                let a = instruction[1];
                let b = before[instruction[2]];
                res[instruction[3]] = (a > b) as usize;
            },
            OpCode::Gtri => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = (a > b) as usize;
            },
            OpCode::Gtrr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = (a > b) as usize;
            },
            OpCode::Eqir => {
                let a = instruction[1];
                let b = before[instruction[2]];
                res[instruction[3]] = (a == b) as usize;
            },
            OpCode::Eqri => {
                let a = before[instruction[1]];
                let b = instruction[2];
                res[instruction[3]] = (a == b) as usize;
            },
            OpCode::Eqrr => {
                let a = before[instruction[1]];
                let b = before[instruction[2]];
                res[instruction[3]] = (a == b) as usize;
            },
        }
        for (i, e) in res.iter().enumerate() {
            if e != &after[i] {
                return false;
            }
        }

        true
    }
}

fn get_matching(instruction: &Vec<usize>, before: &Vec<usize>, after: &Vec<usize>) -> HashSet<OpCode> {
    let mut codes = HashSet::new();
    let enums = [OpCode::Addr, OpCode::Addi, OpCode::Mulr, OpCode::Muli, OpCode::Banr, OpCode::Bani, OpCode::Borr, OpCode::Bori, OpCode::Setr, OpCode::Seti, OpCode::Gtir, OpCode::Gtri, OpCode::Gtrr, OpCode::Eqir, OpCode::Eqri, OpCode::Eqrr];

    for e in enums {
        if e.execute(instruction, before, after) {
            codes.insert(e);
        }
    }

    codes
}

#[test]
fn test() {
    let s = "Before: [instruction[3], 2, 1, 1]
9 2 1 2
After:  [instruction[3], 2, 2, 1]";
    let expected = vec![OpCode::Mulr, OpCode::Addi, OpCode::Seti];
    let got = get_matching(&vec![9, 2, 1, 2], &vec![3, 2, 1, 1], &vec![3, 2, 2, 1]);

    for e in expected.iter() {
        assert!(got.iter().any(|x| x == e))
    }
}