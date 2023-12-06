use crate::util::parse::*;
use itertools::Itertools;

pub struct Input {
    races: Vec<[u64; 2]>,
}

#[aoc_generator(day06)]
pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.lines().collect();
    let times: Vec<u64> = chunks[0].iter_unsigned().collect();
    let dist: Vec<u64> = chunks[1].iter_unsigned().collect();

    Input {
        races: times.iter().zip(dist).map(|(&t, d)| [t, d]).collect(),
    }
}

#[aoc(day06, part1)]
pub fn part1(input: &Input) -> usize {
    let mut total: usize = 1;
    for &[time, dist] in &input.races {
        total *= (0..time)
            .map(|t| (time - t) * t)
            .filter(|d| d > &dist)
            .count()
    }
    total
}

#[aoc(day06, part2)]
pub fn part2(input: &Input) -> usize {
    let time = get_number(input, 0);
    let dist = get_number(input, 1);

    (0..time)
        .map(|t| (time - t) * t)
        .filter(|d| d > &dist)
        .count()
}

fn get_number(input: &Input, index: usize) -> u64 {
    input
        .races
        .iter()
        .map(|x| x[index].to_string())
        .join("")
        .parse()
        .unwrap()
}

#[test]
fn test_part1() {
    let s = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(part1(&parse(s)), 288);
}

#[test]
fn test_part2() {
    let s = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(part2(&parse(s)), 71503);
}
