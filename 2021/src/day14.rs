use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn generator(input: &str) -> (Vec<char>, Vec<Rule>) {
    let input = input.split("\n\n").collect_vec();
    let template: Vec<char> = input[0].chars().collect();
    let rules = input[1]
        .lines()
        .map(|line| {
            let line = line.split(" -> ").collect_vec();
            let first = line[0].chars().collect_vec();
            let second = line[1].chars().next().unwrap();
            ((first[0], first[1]), second)
        })
        .collect_vec();
    (template, rules)
}

#[aoc(day14, part1)]
pub fn part1((polymer, rules): &(Vec<char>, Vec<Rule>)) -> usize {
    execute(polymer, rules, 10)
}

#[aoc(day14, part2)]
pub fn part2((polymer, rules): &(Vec<char>, Vec<Rule>)) -> usize {
    execute(polymer, rules, 40)
}

type Rule = ((char, char), char);

fn execute(polymer: &[char], rules: &[Rule], steps: usize) -> usize {
    let mut map = HashMap::new();
    for i in 0..polymer.len() - 1 {
        let entry = map.entry((polymer[i], polymer[i + 1])).or_insert(0);
        *entry += 1;
    }

    for _ in 0..steps {
        let mut new_map = HashMap::new();

        for (k, _) in map.iter() {
            let rule = rules.iter().find(|r| r.0 == *k).unwrap();
            let entry = new_map.entry((k.0, rule.1)).or_insert(0);
            *entry += map[k];
            let entry = new_map.entry((rule.1, k.1)).or_insert(0);
            *entry += map[k];
        }

        map = new_map;
    }

    let mut letters = HashMap::new();

    for (k, v) in map {
        let entry = letters.entry(k.0).or_insert(0);
        *entry += v;
    }

    let entry = letters.entry(*polymer.last().unwrap()).or_insert(0);
    *entry += 1;

    let max = letters.iter().max_by_key(|(_, v)| *v).unwrap().1;
    let min = letters.iter().min_by_key(|(_, v)| *v).unwrap().1;

    max - min
}

#[test]
fn test() {
    let s = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    assert_eq!(part1(&generator(s)), 1588);
}

#[test]
#[ignore]
fn test2() {
    let s = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    assert_eq!(part2(&generator(s)), 2188189693529);
}
