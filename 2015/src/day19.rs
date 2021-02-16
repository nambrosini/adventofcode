use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type MapType = (String, HashMap<String, Vec<String>>);

#[aoc_generator(day19)]
pub fn generator(input: &str) -> MapType {
    let split = input.split("\n\n").collect_vec();
    let molecule = split[1].to_owned();

    let v: Vec<(String, String)> = split[0]
        .lines()
        .map(|x| x.split(" => ").collect_vec())
        .map(|x| (x[0].to_owned(), x[1].to_owned()))
        .collect_vec();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in v {
        let entry = map.entry(k).or_default();
        entry.push(v);
    }

    (molecule, map)
}

#[aoc(day19, part1)]
pub fn part1(map: &MapType) -> usize {
    let mut set = HashSet::new();

    for (k, v) in &map.1 {
        for v in v {
            if k.len() == 1 {
                for i in 0..map.0.len() {
                    if &map.0[i..=i] == &k[..] {
                        let s = format!(
                            "{}{}{}",
                            map.0[..i].to_owned(),
                            v,
                            map.0[(i + 1)..].to_owned()
                        )
                        .to_owned();
                        set.insert(s);
                    }
                }
            } else {
                for i in 0..map.0.len() - 1 {
                    if &map.0[i..=(i + 1)] == &k[..] {
                        let s = format!(
                            "{}{}{}",
                            map.0[..i].to_owned(),
                            v,
                            map.0[(i + 2)..].to_owned()
                        )
                        .to_owned();
                        set.insert(s);
                    }
                }
            }
        }
    }

    set.len()
}

#[aoc(day19, part2)]
pub fn part2(map: &MapType) -> usize {
    let mut replacements = vec![];
    for (k, val) in &map.1 {
        for e in val {
            replacements.push((k.clone(), e));
        }
    }

    replacements.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut total = 0;
    let mut medicine = map.0.clone();

    while medicine != String::from("e") {
        for (rhs, lhs) in &replacements {
            if medicine.contains(*lhs) {
                medicine = medicine.replacen(*lhs, rhs, 1);
                total += 1;
                break;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = std::fs::read_to_string("tests/day19.txt").unwrap();
        let input = generator(&s);
        assert_eq!(part2(&input), 6);
    }
}
