use itertools::Itertools;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Monkeys {
    input.split("\r\n\r\n").map(|m| m.into()).collect()
}

#[aoc(day11, part1)]
pub fn part1(monkeys: &Monkeys) -> usize {
    let mut monkeys = monkeys.clone();
    monkeys.simulate(3, 20, &Operator::Division)
}

#[aoc(day11, part2)]
pub fn part2(monkeys: &Monkeys) -> usize {
    let div = monkeys.iter().map(|m| m.test).product();
    let mut monkeys = monkeys.clone();
    monkeys.simulate(div, 10_000, &Operator::Modulo)
}

type Monkeys = Vec<Monkey>;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: String,
    test: usize,
    t: usize,
    f: usize,
    count: usize,
}

enum Operator {
    Division,
    Modulo,
}

trait Simulate {
    fn simulate(&mut self, div: usize, cycles: usize, operator: &Operator) -> usize;
}

impl Simulate for Monkeys {
    fn simulate(&mut self, div: usize, cycles: usize, operation: &Operator) -> usize {
        for _ in 0..cycles {
            for i in 0..self.len() {
                let mut monkey = self[i].clone();
                monkey.simulate(operation, div, self);
                self[i] = monkey;
            }
        }

        self.iter()
            .map(|m| m.count)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

impl Monkey {
    fn new(items: Vec<usize>, operation: String, test: usize, t: usize, f: usize) -> Self {
        Self {
            items,
            operation,
            test,
            t,
            f,
            count: 0,
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

    fn simulate(&mut self, operation: &Operator, div: usize, monkeys: &mut [Monkey]) {
        self.count += self.items.len();
        while !self.items.is_empty() {
            let item = self.items.remove(0);
            let item = match operation {
                Operator::Division => Monkey::calc(item, &self.operation) / div,
                Operator::Modulo => Monkey::calc(item, &self.operation) % div,
            };
            let destination_monkey_index = if item % self.test == 0 {
                self.t
            } else {
                self.f
            };
            monkeys[destination_monkey_index].items.push(item);
        }
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
