use std::convert::{TryFrom, TryInto};

pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        Ok(match val {
            "turn on" => Self::TurnOn,
            "toggle" => Self::Toggle,
            "turn off" => Self::TurnOff,
            _ => unreachable!(),
        })
    }
}
pub struct Instruction {
    from: (usize, usize),
    to: (usize, usize),
    action: Action,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = val.split_whitespace().collect();

        if split.len() == 4 {
            let from: Vec<&str> = split[1].split(',').collect();
            let from = (from[0].parse().unwrap(), from[1].parse().unwrap());
            let to: Vec<&str> = split.last().unwrap().split(',').collect();
            let to = (to[0].parse().unwrap(), to[1].parse().unwrap());
            Ok(Self {
                from,
                to,
                action: split[0].try_into().unwrap(),
            })
        } else {
            let from: Vec<&str> = split[2].split(',').collect();
            let from = (from[0].parse().unwrap(), from[1].parse().unwrap());
            let to: Vec<&str> = split.last().unwrap().split(',').collect();
            let to = (to[0].parse().unwrap(), to[1].parse().unwrap());
            Ok(Self {
                from,
                to,
                action: (&format!("{} {}", split[0], split[1])[..])
                    .try_into()
                    .unwrap(),
            })
        }
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| x.try_into().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let mut map = vec![vec![0; 1000]; 1000];
    for i in input {
        for x in map.iter_mut().take(i.to.0).skip(i.from.0) {
            for y in x.iter_mut().take(i.to.1 + 1).skip(i.from.1) {
                *y = match i.action {
                    Action::TurnOff => 0,
                    Action::TurnOn => 1,
                    Action::Toggle => match y {
                        1 => 0,
                        0 => 1,
                        _ => unreachable!(),
                    },
                }
            }
        }
    }

    map.iter().flatten().filter(|x| x == &&1).count()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut map = vec![vec![0; 1000]; 1000];
    for i in input {
        for x in map.iter_mut().take(i.to.0 + 1).skip(i.from.0) {
            for y in x.iter_mut().take(i.to.1 + 1).skip(i.from.1) {
                *y += match i.action {
                    Action::TurnOff => {
                        if *y == 0 {
                            0
                        } else {
                            -1
                        }
                    }
                    Action::TurnOn => 1,
                    Action::Toggle => 2,
                }
            }
        }
    }

    map.iter().flatten().sum()
}
