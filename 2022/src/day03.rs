use std::collections::HashSet;

#[aoc_generator(day3, part1)]
pub fn generator_part1(input: &str) -> Vec<(Compartment, Compartment)> {
    input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let mid = chars.len() / 2;
            (
                chars[..mid].iter().copied().collect(),
                chars[mid..].iter().copied().collect(),
            )
        })
        .collect()
}

#[aoc_generator(day3, part2)]
pub fn generator_part2(input: &str) -> Vec<Compartment> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day03, part1)]
pub fn part1(input: &[(Compartment, Compartment)]) -> usize {
    input
        .iter()
        .filter_map(|(a, b)| a.intersection(b).next().map(|item| item.calc()))
        .sum()
}

#[aoc(day03, part2)]
pub fn part2(input: &[Compartment]) -> usize {
    input
        .chunks(3)
        .filter_map(|group| {
            group[0]
                .iter()
                .find(|item| group[1].contains(item) && group[2].contains(item))
        })
        .map(|item| item.calc())
        .sum()
}

type Compartment = HashSet<char>;

trait Priority {
    fn calc(&self) -> usize;
}

impl Priority for char {
    fn calc(&self) -> usize {
        if (b'A'..=b'Z').contains(&(*self as u8)) {
            (*self as u8 - b'A' + 27) as usize
        } else {
            (*self as u8 - b'a' + 1) as usize
        }
    }
}

#[test]
fn test() {
    let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let want = 157;
    let got = part1(&generator_part1(s));

    assert_eq!(want, got);
}

#[test]
fn test2() {
    let s = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let want = 70;
    let got = part2(&generator_part2(s));

    assert_eq!(want, got);
}
