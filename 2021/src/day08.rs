use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;

#[aoc_generator(day08, part1)]
pub fn generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split('|')
                .last()
                .unwrap()
                .split(' ')
                .map(|x| x.to_owned())
                .collect()
        })
        .collect_vec()
}

#[aoc_generator(day08, part2)]
pub fn generator2(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[aoc(day08, part1)]
pub fn part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .flatten()
        .filter(|x| x.len() == 3 || x.len() == 4 || x.len() == 2 || x.len() == 7)
        .count()
}

#[aoc(day08, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut numbers: Vec<HashSet<char>> = Vec::new();
    (0..=9).for_each(|_i| numbers.push(HashSet::new()));
    input
        .iter()
        .map(|line| {
            let line_chain = line
                .input
                .iter()
                .chain(line.output.iter())
                .map(|word| word.chars().collect::<HashSet<char>>());
            numbers[1] = line_chain.clone().find(|word| word.len() == 2).unwrap();
            numbers[7] = line_chain.clone().find(|word| word.len() == 3).unwrap();
            numbers[4] = line_chain.clone().find(|word| word.len() == 4).unwrap();
            numbers[3] = line_chain
                .clone()
                .filter(|word| word.len() == 5)
                .find(|chars| chars.is_superset(&numbers[1]))
                .unwrap();
            numbers[9] = line_chain
                .clone()
                .filter(|word| word.len() == 6)
                .find(|chars| chars.is_superset(&numbers[3]))
                .unwrap();
            numbers[8] = line_chain.clone().find(|word| word.len() == 7).unwrap();
            let bottom_left = *(&numbers[8] - &numbers[9]).iter().next().unwrap();
            numbers[2] = line_chain
                .clone()
                .filter(|word| word.len() == 5)
                .find(|chars| chars.contains(&bottom_left))
                .unwrap();
            numbers[5] = line_chain
                .clone()
                .filter(|word| word.len() == 5)
                .find(|chars| chars != &numbers[2] && chars != &numbers[3])
                .unwrap();
            numbers[6] = line_chain
                .clone()
                .filter(|word| word.len() == 6)
                .find(|chars| chars != &numbers[9] && chars.is_superset(&numbers[5]))
                .unwrap();
            numbers[0] = line_chain
                .clone()
                .filter(|word| word.len() == 6)
                .find(|chars| chars != &numbers[9] && chars != &numbers[6])
                .unwrap();

            line.output
                .iter()
                .map(|word| word.chars().collect::<HashSet<char>>())
                .map(|chars| {
                    numbers
                        .iter()
                        .enumerate()
                        .find(|(_i, digits)| chars == **digits)
                        .map(|(i, _digits)| i)
                        .expect("Could not find a matching digit")
                })
                .rev()
                .enumerate()
                .map(|(e, digit)| ((digit as usize) * 10_usize.pow(e.try_into().unwrap())))
                .sum::<usize>()
        })
        .sum()
}

pub struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let split = s.split(" | ").collect_vec();
        let input = split[0].split(' ').map(|x| x.to_string()).collect_vec();
        let output = split[1].split(' ').map(|x| x.to_string()).collect_vec();

        Self { input, output }
    }
}

#[test]
fn test() {
    assert_eq!("abc".chars().collect_vec(), vec!['a', 'b', 'c']);
}
