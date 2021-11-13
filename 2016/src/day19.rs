use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day19)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day19, part1)]
pub fn part1(input: &usize) -> usize {
    solve(*input)
}

#[aoc(day19, part2)]
pub fn part2(_input: &usize) -> usize {
    0
}

fn solve(input: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 1..=input {
        map.insert(i, 1);
    }

    while map.len() > 1 {
        map = take_present(&map);
        map = purge_no_presents(&map);
    }

    *map.iter().next().unwrap().0
}

fn take_present(map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_map = map.clone();
    let mut keys = map.keys().collect_vec();
    keys.sort_unstable();

    for (i, k) in keys.iter().enumerate() {
        if new_map[k] == 0 {
            continue;
        }

        let mut index = (i + 1) % keys.len();

        while index != i {
            if new_map[keys[index]] != 0 {
                let entry = new_map.entry(**k).or_default();
                *entry += map[keys[index]];
                let entry = new_map.entry(*keys[index]).or_default();
                *entry = 0;
                break;
            }

            index = (index + 1) % keys.len();
        }
    }

    new_map
}

fn purge_no_presents(map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_map = map.clone();
    let keys = map.keys().collect_vec();

    for k in keys {
        if map[k] == 0 {
            new_map.remove(k);
        }
    }

    new_map
}

#[test]
fn test() {
    assert_eq!(part1(&5), 3);
}