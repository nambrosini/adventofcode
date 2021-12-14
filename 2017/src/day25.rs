use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day25)]
pub fn generator(input: &str) -> (usize, HashMap<char, (Instruction, Instruction)>) {
    let s = input.split("\n\n").collect_vec();

    // steps
    let first = s[0].lines().last().unwrap();
    let re = Regex::new(r"(\d+)").unwrap();
    let steps = re.captures(first).unwrap()[0].parse().unwrap();

    let mut map = HashMap::new();
    let re = Regex::new(r"In state ([A-F]):\n(\s|\w|-)+(\d):\n(\s|-|\w)+.\n((right|left)|\s|-|\w)+.\n((right|left)|\s|-|\w)+.\n(\s|\w|-)+(\d):\n(\s|-|\w)+.\n((right|left)|\s|-|\w)+.\n((right|left)|\s|-|\w)+.").unwrap();

    for ins in s.iter().skip(1) {
        let caps = re.captures(ins).unwrap();
        let state = caps[1].chars().next().unwrap();
        let inst1 = Instruction::Zero(
            Op::Write(caps[4].parse().unwrap()),
            Op::Move(caps[5].into()),
            Op::Continue(caps[7].chars().next().unwrap()),
        );
        let inst2 = Instruction::One(
            Op::Write(caps[11].parse().unwrap()),
            Op::Move(caps[12].into()),
            Op::Continue(caps[14].chars().next().unwrap())
        );

        map.insert(state, (inst1, inst2));
    }

    (steps, map)
}

#[aoc(day25, part1)]
pub fn part1((steps, map): &(usize, HashMap<char, (Instruction, Instruction)>)) -> usize {
    let mut current_state = 'A';
    let mut current_position: i32 = 0;
    let mut tape: HashMap<i32, usize> = HashMap::new();

    for _ in 0..*steps {
        let entry = tape.entry(current_position).or_insert(0);

        if *entry == 0 {
            let instruction = &map[&current_state].0;
            if let Instruction::Zero(Op::Write(v), Op::Move(m), Op::Continue(c)) = instruction {
                *entry = *v;
                current_position += i32::from(*m);
                current_state = *c;
            } else {
                unreachable!();
            }
        } else {
            let instruction = &map[&current_state].1;
            if let Instruction::One(Op::Write(v), Op::Move(m), Op::Continue(c)) = instruction {
                *entry = *v;
                current_position += i32::from(*m);
                current_state = *c;
            } else {
                unreachable!();
            }
        }
    }

    tape.values().filter(|&&x| x == 1).count()
}

pub enum Instruction {
    Zero(Op, Op, Op),
    One(Op, Op, Op)
}

pub enum Op {
    Write(usize),
    Move(Direction),
    Continue(char)
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right
}

impl From<Direction> for i32 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "left" => Direction::Left,
            "right" => Direction::Right,
            v => {
                println!("{}", v);
                unreachable!();
            }
        }
    }
}

#[test]
fn test() {
    let s = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
    If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
    If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
    If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
    If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    assert_eq!(part1(&generator(s)), 3);
}