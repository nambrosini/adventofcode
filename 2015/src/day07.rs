use std::collections::HashMap;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day07, part1)]
pub fn part1(instructions: &[Instruction]) -> u16 {
    let mut instructions = instructions.to_vec();
    println!("{:?}", instructions.len());
    let mut map: HashMap<String, u16> = HashMap::new();

    while !instructions.is_empty() {
        let instruction = instructions.remove(0);
        let mut executed = false;

        match &instruction {
            Instruction::Direct(p, r) => {
                if let Some(p1) = p.get_value(&map) {
                    if p1 == 1674 {
                        println!("{}", instructions.len());
                    }
                    map.insert(r.clone(), p1);
                    executed = true;
                }
            }
            Instruction::And(p1, p2, r) => {
                if let (Some(p1), Some(p2)) = (p1.get_value(&map), p2.get_value(&map)) {
                    map.insert(r.clone(), p1 & p2);
                    executed = true;
                }
            }
            Instruction::Lshift(p1, p2, r) => {
                if let (Some(p1), Some(p2)) = (p1.get_value(&map), p2.get_value(&map)) {
                    map.insert(r.clone(), p1 << p2);
                    executed = true;
                }
            }
            Instruction::Not(p1, r) => {
                if let Some(p1) = p1.get_value(&map) {
                    map.insert(r.clone(), !p1);
                    executed = true;
                }
            }
            Instruction::Or(p1, p2, r) => {
                if let (Some(p1), Some(p2)) = (p1.get_value(&map), p2.get_value(&map)) {
                    map.insert(r.clone(), p1 | p2);
                    executed = true;
                }
            }
            Instruction::Rshift(p1, p2, r) => {
                if let (Some(p1), Some(p2)) = (p1.get_value(&map), p2.get_value(&map)) {
                    map.insert(r.clone(), p1 >> p2);
                    executed = true;
                }
            }
        }

        if !executed {
            instructions.push(instruction);
        }
    }

    println!("{:?}", map);
    map["a"]
}
//
// #[aoc(day07, part2)]
// pub fn part2(instructions: &[Instruction]) -> usize {
//     0
// }

#[derive(Debug, Clone)]
pub enum Instruction {
    Direct(Signal, String),
    And(Signal, Signal, String),
    Lshift(Signal, Signal, String),
    Not(Signal, String),
    Or(Signal, Signal, String),
    Rshift(Signal, Signal, String),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split_whitespace().collect();
        match split.len() {
            3 => Self::Direct(split[0].into(), split[2].into()),
            4 => Self::Not(split[1].into(), split[3].into()),
            5 => match split[1] {
                "AND" => Self::And(split[0].into(), split[2].into(), split[4].into()),
                "OR" => Self::Or(split[0].into(), split[2].into(), split[4].into()),
                "LSHIFT" => Self::Lshift(split[0].into(), split[2].into(), split[4].into()),
                "RSHIFT" => Self::And(split[0].into(), split[2].into(), split[4].into()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Signal {
    Value(u16),
    Wire(String),
}

impl Signal {
    fn get_value(&self, map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Signal::Value(v) => Some(*v),
            Signal::Wire(s) => map.get(s).copied(),
        }
    }
}

impl From<&str> for Signal {
    fn from(value: &str) -> Self {
        if let Ok(v) = value.parse() {
            Self::Value(v)
        } else {
            Self::Wire(value.to_string())
        }
    }
}

#[test]
fn test1() {
    let s = "123 -> a
a AND 10 -> a
a LSHIFT 1 -> a
NOT a -> a";

    assert_eq!(20, part1(&generator(s)))
}
