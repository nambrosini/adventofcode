use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.to_owned()).collect_vec())
        .collect_vec()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<String>]) -> i32 {
    let count: Vec<usize> = input
        .iter()
        .map(|l| l.len() - l.iter().cloned().collect::<HashSet<String>>().len())
        .collect_vec();

    count
        .iter()
        .fold(0, |sum, c| sum + if *c == 0usize { 1 } else { 0 })
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<String>]) -> i32 {
    let count: Vec<usize> = input
        .iter()
        .map(|l| {
            l.iter()
                .map(|l| {
                    let slice = &l[..];
                    let mut chars: Vec<char> = slice.to_owned().chars().collect();
                    chars.sort_by(|a, b| b.cmp(a));
                    String::from_iter(chars)
                })
                .collect_vec()
        })
        .map(|l| l.len() - l.iter().cloned().collect::<HashSet<String>>().len())
        .collect_vec();

    count
        .iter()
        .fold(0, |sum, c| sum + if *c == 0usize { 1 } else { 0 })
}
