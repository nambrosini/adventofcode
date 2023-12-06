use crate::util::parse::*;

pub struct Input1 {
    races: Vec<[u64; 2]>,
}

pub struct Input2 {
    time: u64,
    dist: u64,
}

#[aoc_generator(day06, part1)]
pub fn parse1(input: &str) -> Input1 {
    let chunks: Vec<_> = input.lines().collect();
    let times: Vec<u64> = chunks[0].iter_unsigned().collect();
    let dist: Vec<u64> = chunks[1].iter_unsigned().collect();

    Input1 {
        races: times.iter().zip(dist).map(|(&t, d)| [t, d]).collect(),
    }
}

#[aoc(day06, part1)]
pub fn part1(input: &Input1) -> usize {
    let mut total: usize = 1;
    for &[time, dist] in &input.races {
        total *= (0..time)
            .map(|t| (time - t) * t)
            .filter(|d| d > &dist)
            .count()
    }
    total
}

#[aoc_generator(day06, part2)]
pub fn parse2(input: &str) -> Input2 {
    let chunks: Vec<_> = input.lines().collect();
    let time: u64 = chunks[0]
        .replace(' ', "")
        .as_str()
        .iter_unsigned()
        .next()
        .unwrap();
    let dist: u64 = chunks[1]
        .replace(' ', "")
        .as_str()
        .iter_unsigned()
        .next()
        .unwrap();

    Input2 { time, dist }
}

#[aoc(day06, part2)]
pub fn part2(input: &Input2) -> usize {
    (0..input.time)
        .map(|t| (input.time - t) * t)
        .filter(|d| d > &input.dist)
        .count()
}

#[test]
fn test_part1() {
    let s = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(part1(&parse1(s)), 288);
}

#[test]
fn test_part2() {
    let s = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(part2(&parse2(s)), 71503);
}
