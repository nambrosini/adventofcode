use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split("\n\n")
        .map(|x| x.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    *input.iter()
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    input.iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}