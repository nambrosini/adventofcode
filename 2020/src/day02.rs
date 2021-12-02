use lazy_static::lazy_static;
use std::convert::{TryFrom, TryInto};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Password {
    first: usize,
    last: usize,
    letter: char,
    password: String,
}

impl Password {
    fn check_policy1(self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();

        count >= self.first && count <= self.last
    }

    fn check_policy2(self) -> bool {
        let first_letter = self.password.chars().nth(self.first - 1).unwrap();
        let last_letter = self.password.chars().nth(self.last - 1).unwrap();

        first_letter == self.letter && last_letter != self.letter
            || first_letter != self.letter && last_letter == self.letter
    }
}

impl TryFrom<&str> for Password {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
        }

        Ok(RE
            .captures(value)
            .map(|cap| Password {
                first: cap
                    .get(1)
                    .map(|first| first.as_str().parse().unwrap())
                    .unwrap(),
                last: cap
                    .get(2)
                    .map(|last| last.as_str().parse().unwrap())
                    .unwrap(),
                letter: cap
                    .get(3)
                    .map(|letter| letter.as_str().chars().next().unwrap())
                    .unwrap(),
                password: cap
                    .get(4)
                    .map(|password| password.as_str())
                    .unwrap()
                    .to_owned(),
            })
            .unwrap())
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Password> {
    input.lines().map(|l| l.try_into().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Password]) -> usize {
    input.iter().fold(0, |count, x| {
        count + if x.clone().check_policy1() { 1 } else { 0 }
    })
}

#[aoc(day2, part2)]
pub fn part2(input: &[Password]) -> usize {
    input.iter().fold(0, |count, x| {
        count + if x.clone().check_policy2() { 1 } else { 0 }
    })
}

#[test]
fn sample1() {
    let s = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    assert_eq!(part1(&generator(s)), 2);
}

#[test]
fn sample2() {
    let s = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    assert_eq!(part2(&generator(s)), 1);
}
