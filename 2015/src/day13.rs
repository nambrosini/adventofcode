use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type MapType = HashMap<String, HashMap<String, i32>>;

#[aoc_generator(day13)]
pub fn generator(input: &str) -> MapType {
    let lines = input.lines();
    let mut map: MapType = HashMap::new();

    for next in lines {
        let split = next.split(' ').collect_vec();
        let name = split[0];
        let points: i32 = match split[2] {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!()
        } * split[3].parse::<i32>().unwrap();
        let neighbour = split.iter().last().unwrap().replace(".", "");

        let entry = map.entry(name.to_owned()).or_default();
        entry.insert(neighbour, points);
    }

    map
}

#[aoc(day13, part1)]
pub fn part1(map: &MapType) -> i32 {
    let happiness = calc_happiness(map);

    *happiness.iter().max().unwrap()
}

#[aoc(day13, part2)]
pub fn part2(map: &MapType) -> i32 {
    let mut map = map.clone();
    let mut my = HashMap::new();

    for p in map.keys() {
        my.insert(p.to_owned(), 0);
    }

    for (k, v) in map.iter_mut() {
        v.insert("Me".to_owned(), 0);
    }

    map.insert("Me".to_owned(), my);

    let happiness = calc_happiness(&map);

    *happiness.iter().max().unwrap()
}

fn calc_happiness(map: &MapType) -> HashSet<i32> {
    let perm = map.keys().permutations(map.len()).collect_vec();

    let mut set = HashSet::new();

    for p in perm {
        let mut sum = 0;

        for i in 0..p.len() {
            sum += map[p[i]][p[(i + 1) % p.len()]];
            let rev = p.len() - 1 - i;
            sum += map[p[rev]][p[
                match rev.checked_sub(1) {
                    Some(v) => v,
                    None => p.len() - 1
                }
                ]];
        }

        set.insert(sum);
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = std::fs::read_to_string("tests/day13/sample1.txt").unwrap();

        let generated = generator(&s);

        assert_eq!(part1(&generated), 330);
    }
}
