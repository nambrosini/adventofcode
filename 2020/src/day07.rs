use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[String]) -> usize {
    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for s in input {
        let s = s.to_owned().replace("bags", "");
        let s = s.replace("bag", "");
        let s = s.replace(".", "");
        let s: Vec<String> = s.trim().split("contain ").map(|s| s.to_owned()).collect();

        let bag_color = s[0].trim();

        let s: Vec<String> = s[1].split(',').map(|s| s.trim().to_owned()).collect();

        for i in &s {
            let n: usize = if i == "no other" {
                continue;
            } else {
                i[..1].to_owned().parse().unwrap()
            };

            let key = i[2..].trim().to_owned();

            let bag = bags.entry(key.to_owned()).or_default();
            bag.insert(bag_color.to_owned(), n);
        }
    }

    let mut bags_containing_gold: Vec<String> = vec!["shiny gold".to_owned()];

    let mut counter = 0;

    while counter < bags_containing_gold.len() {
        if let Some(v) = bags.get(&bags_containing_gold[counter]) {
            bags_containing_gold.append(&mut v.keys().map(|s| s.to_owned()).collect());
        }
        counter += 1;
    }

    HashSet::<&String>::from_iter(bags_containing_gold.iter().clone()).len() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &[String]) -> usize {
    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for s in input {
        let s = s.to_owned().replace("bags", "");
        let s = s.replace("bag", "");
        let s = s.replace(".", "");
        let s: Vec<String> = s.trim().split("contain ").map(|s| s.to_owned()).collect();

        let bag_color = s[0].trim();

        let s: Vec<String> = s[1].split(',').map(|s| s.trim().to_owned()).collect();

        for i in s {
            let n: usize = if i == "no other" {
                continue;
            } else {
                i[..1].to_owned().parse().unwrap()
            };

            let key = i[2..].trim().to_owned();

            let bag = bags.entry(bag_color.to_owned()).or_default();
            bag.insert(key.to_owned(), n);
        }
    }

    calc_recursive("shiny gold", &bags)
}

pub fn calc_recursive(bag_name: &str, bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    if let Some(v) = bags.get(bag_name) {
        let mut res = 0;
        for bag in v {
            res += bag.1 * calc_recursive(bag.0, bags) + bag.1;
        }

        res
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = generator(&std::fs::read_to_string("tests/day07/sample1").unwrap());

        assert_eq!(part1(&s), 4);
    }

    #[test]
    fn sample1_part2() {
        let s = generator(&std::fs::read_to_string("tests/day07/sample1").unwrap());

        assert_eq!(part2(&s), 32);
    }

    #[test]
    fn sample2_part2() {
        let s = generator(&std::fs::read_to_string("tests/day07/sample2").unwrap());

        assert_eq!(part2(&s), 126);
    }
}
