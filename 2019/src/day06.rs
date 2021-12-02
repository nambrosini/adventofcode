use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day06)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for i in input.lines() {
        let split = i.split(')').collect_vec();
        let k = split[0].to_owned();

        let e = map.entry(k).or_default();
        e.push(split[1].to_string());
    }

    map
}

#[aoc(day06, part1)]
pub fn part1(input: &HashMap<String, Vec<String>>) -> usize {
    count_orbit("COM", input, 0)
}

#[aoc(day06, part2)]
pub fn part2(input: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    transfer("YOU", "SAN", 0, input, &mut visited) - 2
}

fn transfer(start: &str, end: &str, count: usize, map: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) -> usize {
    if start == end {
        return count;
    }

    visited.insert(start.to_string());

    let mut children: Vec<String> = vec![];

    if map.contains_key(start) {
        for c in &map[start] {
            children.push(c.to_string());
        }
    }

    if let Some(k) = map.iter().find(|(_, v)| v.contains(&start.to_string())) {
        children.push(k.0.to_string());
    }

    if children.is_empty() {
        return count;
    }

    let mut min = usize::MAX;

    for c in children.iter() {
        if visited.contains(c) {
            continue;
        }
        let val = transfer(c, end, count + 1, map, visited);

        if val < min {
            min = val;
        }
    }

    min
}

fn count_orbit(start: &str, map: &HashMap<String, Vec<String>>, orbits: usize) -> usize {
    if !map.contains_key(start) {
        return orbits;
    }

    let mut sum = 0;

    for v in &map[start] {
        sum += count_orbit(v, map, orbits + 1);
    }

    sum + orbits
}

#[test]
fn test1() {
    let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    assert_eq!(part1(&generator(s)), 42);
}

#[test]
fn test2() {
    let s = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
    let s = generator(s);

    assert_eq!(part2(&s), 4);
}