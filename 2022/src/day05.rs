use itertools::Itertools;

type State = Vec<Vec<char>>;

#[aoc_generator(day05)]
pub fn generator(input: &str) -> (State, Vec<Instruction>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let chars: Vec<Vec<char>> = split[0].lines().map(|l| l.chars().collect()).collect();

    let mut state: State = vec![vec![]; chars.last().unwrap().len() / 4 + 1];

    'outer: for row in chars.iter() {
        let mut col = 1;
        while col < row.len() {
            if char::is_digit(row[col], 10) {
                break 'outer;
            } else if row[col] != ' ' {
                state[col / 4].insert(0, row[col]);
            }

            col += 4;
        }
    }

    let instructions = split[1].lines().map(|l| l.into()).collect();

    (state, instructions)
}

#[aoc(day05, part1)]
pub fn part1(input: &(State, Vec<Instruction>)) -> String {
    let mut state = input.0.clone();
    for instruction in &input.1 {
        for _ in 0..instruction.n {
            let c = state[instruction.from - 1].pop().unwrap();
            state[instruction.to - 1].push(c);
        }
    }

    state.iter().map(|t| t.last().unwrap()).join("")
}

#[aoc(day05, part2)]
pub fn part2(input: &(State, Vec<Instruction>)) -> String {
    let mut state = input.0.clone();
    for instruction in &input.1 {
        let len = state[instruction.from - 1].len();
        let mut v: Vec<char> = state[instruction.from - 1]
            .drain(len - instruction.n..)
            .collect();
        state[instruction.to - 1].append(&mut v);
    }

    state.iter().map(|t| t.last().unwrap()).join("")
}

pub struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        let n = split[1].parse().unwrap();
        let from = split[3].parse().unwrap();
        let to = split[5].parse().unwrap();
        Self { n, from, to }
    }
}

#[test]
fn test() {
    let s = std::fs::read_to_string("tests/test05.txt").unwrap();
    let got = part1(&generator(&s));

    assert_eq!("CMZ", got);
}

#[test]
fn test2() {
    let s = std::fs::read_to_string("tests/test05.txt").unwrap();
    let got = part2(&generator(&s));

    assert_eq!("MCD", got);
}
