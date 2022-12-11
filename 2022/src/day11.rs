use itertools::Itertools;
use std::fmt::{Display, Formatter};

type Monkeys = Vec<Monkey>;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Monkeys {
    input.split("\n\n").map(|m| m.into()).collect()
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &Monkeys) -> usize {
    simulate(monkeys, 3, 20, "/")
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &Monkeys) -> usize {
    let div = monkeys.iter().map(|m| m.test).product();
    simulate(monkeys, div, 10_000, "%")
}

fn simulate(monkeys: &Monkeys, div: usize, cycles: usize, operation: &str) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut counts = vec![0; monkeys.len()];

    for _ in 0..cycles {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            while !monkey.items.is_empty() {
                counts[i] += 1;
                let item = monkey.items.remove(0);
                let item = if operation == "/" {
                    Monkey::calc(item, &monkey.operation) / div
                } else {
                    Monkey::calc(item, &monkey.operation) % div
                };
                if item % monkey.test == 0 {
                    monkeys[monkey.t].items.push(item);
                } else {
                    monkeys[monkey.f].items.push(item);
                }
            }
            monkeys[i] = monkey;
        }
    }

    counts.iter().sorted().rev().take(2).product()
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: String,
    test: usize,
    t: usize,
    f: usize,
}

impl Monkey {
    fn new(items: Vec<usize>, operation: String, test: usize, t: usize, f: usize) -> Self {
        Self {
            items,
            operation,
            test,
            t,
            f,
        }
    }

    fn calc(item: usize, operation: &str) -> usize {
        let operation: Vec<&str> = operation.split_whitespace().collect();
        let first = if let Ok(val) = operation[0].parse() {
            val
        } else {
            item
        };
        let second = if let Ok(val) = operation[2].parse() {
            val
        } else {
            item
        };
        match operation[1] {
            "*" => first * second,
            "+" => first + second,
            _ => unreachable!(),
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.lines().collect();
        let items: Vec<usize> = parts[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let operation = parts[2];
        let operation: String = operation.split("= ").last().unwrap().to_string();
        let test = parts[3].split_whitespace().last().unwrap().parse().unwrap();
        let t = parts[4].split_whitespace().last().unwrap().parse().unwrap();
        let f = parts[5].split_whitespace().last().unwrap().parse().unwrap();

        Self::new(items, operation, test, t, f)
    }
}

#[test]
fn test() {
    let s = std::fs::read_to_string("tests/test11.txt").unwrap();
    let got = part1(&generator(&s));

    assert_eq!(got, 10605)
}

#[test]
fn test2() {
    let s = std::fs::read_to_string("tests/test11.txt").unwrap();
    let got = part2(&generator(&s));

    assert_eq!(got, 2713310158)
}
