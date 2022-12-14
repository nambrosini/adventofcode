use std::{collections::HashMap, fmt::Display};

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Pots {
    input.into()
}

#[aoc(day12, part1)]
pub fn part1(pots: &Pots) -> i64 {
    let mut pots = pots.clone();
    for _ in 0..20 {
        pots.simulate();
    }

    pots.pots
        .iter()
        .filter(|(_, c)| c == &&'#')
        .map(|(i, _)| i)
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(pots: &Pots) -> i64 {
    run(&mut pots.clone(), 50_000_000_000)
}

fn run(pots: &mut Pots, cycles: usize) -> i64 {
    for i in 0..cycles {
        pots.simulate();
        if i == 5 || i == 50 || i == 500 || i == 5000 || i == 50000 {
            println!(
                "\nsum of pots with plants after {} generations: {}",
                i,
                pots.calc()
            );
        }
        print!("\r{}", i);
    }

    pots.calc()
}

#[derive(Clone)]
pub struct Pots {
    pots: HashMap<i64, char>,
    states: Vec<State>,
}

impl Pots {
    fn simulate(&mut self) {
        let mut pots_clone = self.pots.clone();

        let min = *self.pots.keys().min().unwrap();
        let max = *self.pots.keys().max().unwrap();

        for i in min - 2..=max + 2 {
            let mut string = String::new();
            for j in i - 2..=i + 2 {
                if let Some(c) = self.pots.get(&j) {
                    string.push(*c);
                } else {
                    string.push('.');
                }
            }
            let val = self.get_pot_from_states(string);
            if !((i < min || i > max) && val == '.') {
                let entry = pots_clone.entry(i).or_insert('.');
                *entry = val;
            }
        }

        self.pots = pots_clone;
    }

    fn get_pot_from_states(&self, string: String) -> char {
        if let Some(x) = self.states.iter().find(|state| state.left == string) {
            x.right
        } else {
            '.'
        }
    }

    fn calc(&self) -> i64 {
        self.pots
            .iter()
            .filter(|(_, c)| c == &&'#')
            .map(|(i, _)| i)
            .sum()
    }
}

impl From<&str> for Pots {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split("\n\n").collect();

        let pots = split[0]
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .map(|(i, e)| (i as i64, e))
            .collect();
        let states = split[1].lines().map(|l| l.into()).collect();

        Self { pots, states }
    }
}

impl Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = *self.pots.keys().min().unwrap();
        let max = *self.pots.keys().max().unwrap();

        for i in min..=max {
            write!(f, "{}", self.pots[&i])?;
        }
        writeln!(f)
    }
}

#[derive(Clone)]
struct State {
    left: String,
    right: char,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" => ").collect();
        let left = split[0].to_string();
        let right = split[1].chars().next().unwrap();

        Self { left, right }
    }
}

#[test]
fn test() {
    let s = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
    assert_eq!(325, part1(&generator(s)));
}
