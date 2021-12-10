use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day06)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

#[aoc(day06, part1)]
pub fn part1(input: &[usize]) -> usize {
    simulate_opti(input, 80)
}

#[aoc(day06, part2)]
pub fn part2(input: &[usize]) -> usize {
    simulate_opti(input, 256)
}

fn simulate_opti(input: &[usize], days: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in input.iter() {
        let entry = map.entry(*i).or_insert(0);
        *entry += 1;
    }

    for _ in 0..days {
        let mut new_map: HashMap<usize, usize> = HashMap::new();

        for k in 0..=8 {
            if let Some(v) = map.get(&k) {
                if k == 0 {
                    let entry = new_map.entry(6).or_insert(0);
                    *entry += v;
                    let entry = new_map.entry(8).or_insert(0);
                    *entry += v;
                } else {
                    let entry = new_map.entry(k - 1).or_insert(0);
                    *entry += v;
                }
            }
        }

        map = new_map;
    }

    map.values().sum::<usize>()
}

#[test]
fn test() {
    let s = "3,4,3,1,2";

    assert_eq!(simulate_opti(&generator(s), 18), 26);
}

#[test]
fn tes2() {
    let s = "3,4,3,1,2";

    assert_eq!(simulate_opti(&generator(s), 256), 26984457539);
}
