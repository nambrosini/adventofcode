use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{From, Into, TryFrom, TryInto};

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.into()).collect_vec()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut map: HashMap<&str, i32> = HashMap::new();

    for instruction in input {
        let r1 = *map.entry(&instruction.r1).or_insert(0);

        if instruction.operator.compare(r1, instruction.r2) {
            let register = map.entry(&instruction.register).or_insert(0);
            instruction.operation.calc(register)
        }
    }

    *map.values().max().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut map: HashMap<&str, i32> = HashMap::new();
    let mut max = 0;

    for instruction in input {
        let r1 = *map.entry(&instruction.r1).or_insert(0);

        if instruction.operator.compare(r1, instruction.r2) {
            let register = map.entry(&instruction.register).or_insert(0);
            instruction.operation.calc(register)
        }

        let m = *map.values().max().unwrap();

        if m > max {
            max = m;
        }
    }

    max
}

pub struct Instruction {
    register: String,
    operation: Operation,
    r1: String,
    operator: Operator,
    r2: i32,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split = value.split(' ').collect_vec();

        let register = split[0].to_owned();
        let operation = (split[1], split[2]).try_into().unwrap();
        let r1 = split[4].to_owned();
        let operator = split[5].try_into().unwrap();
        let r2 = split[6].parse().unwrap();

        Self {
            register,
            operation,
            r1,
            operator,
            r2,
        }
    }
}

enum Operation {
    Inc(i32),
    Dec(i32),
}

impl Operation {
    fn calc(&self, x: &mut i32) {
        match self {
            Self::Inc(v) => *x += v,
            Self::Dec(v) => *x -= v,
        }
    }
}

impl TryFrom<(&str, &str)> for Operation {
    type Error = String;
    fn try_from(op: (&str, &str)) -> Result<Operation, Self::Error> {
        match op.0 {
            "inc" => Ok(Operation::Inc(op.1.parse().unwrap())),
            "dec" => Ok(Operation::Dec(op.1.parse().unwrap())),
            _ => Err(format!("Nonono: {:?}", op)),
        }
    }
}

enum Operator {
    Gt,
    Lt,
    Geq,
    Eq,
    Leq,
    Neq,
}

impl Operator {
    fn compare(&self, a: i32, b: i32) -> bool {
        match self {
            Operator::Gt => a > b,
            Operator::Lt => a < b,
            Operator::Geq => a >= b,
            Operator::Eq => a == b,
            Operator::Leq => a <= b,
            Operator::Neq => a != b,
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(val: &str) -> Result<Operator, String> {
        match val {
            ">" => Ok(Operator::Gt),
            "<" => Ok(Operator::Lt),
            ">=" => Ok(Operator::Geq),
            "==" => Ok(Operator::Eq),
            "<=" => Ok(Operator::Leq),
            "!=" => Ok(Operator::Neq),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let s = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let s = generator(s);

        assert_eq!(part1(&s), 1);
    }

    #[test]
    fn sample2() {
        let s = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let s = generator(s);

        assert_eq!(part2(&s), 10);
    }
}
