use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::{From, Into};

#[aoc_generator(day12)]
pub fn generator(input: &str) -> HashMap<usize, HashSet<usize>> {
    let input: Vec<Pipe> = input.lines().map(|line| line.into()).collect_vec();

    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for p in input {
        let entry = map.entry(p.name).or_default();
        entry.extend(p.connections.clone());
    }

    map
}

#[aoc(day12, part1)]
pub fn part1(input: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut group_0 = input.get(&0).unwrap().iter().collect_vec();
    group_0.push(&0);

    let mut count = 0;

    while count < group_0.len() {
        let current = group_0[count];
        let set = input.get(current).unwrap();

        for i in set {
            if !group_0.contains(&i) {
                group_0.push(i);
            }
        }

        count += 1;
    }

    group_0.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &HashMap<usize, HashSet<usize>>) -> usize {
    let keys: Vec<usize> = input.keys().copied().collect_vec();

    let mut groups: Vec<Vec<usize>> = vec![];

    for k in keys.iter() {
        if !groups.iter().flatten().copied().any(|x| x == *k) {
            let mut group = input.get(&0).unwrap().iter().copied().collect_vec();
            group.push(*k);

            let mut count = 0;

            while count < group.len() {
                let current = group[count];
                let set = input.get(&current).unwrap();

                for i in set {
                    if !group.contains(i) {
                        group.push(*i);
                    }
                }

                count += 1;
            }
            groups.push(group);
        }
    }

    groups.len()
}

pub struct Pipe {
    name: usize,
    connections: HashSet<usize>,
}

impl From<&str> for Pipe {
    fn from(s: &str) -> Self {
        let split = s.split(" <-> ").collect_vec();
        let name = split[0].parse().unwrap();
        let connections = split[1].split(", ").map(|x| x.parse().unwrap()).collect();
        Self { name, connections }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let s = generator(s);

        assert_eq!(part1(&s), 6);
    }

    #[test]
    fn test2() {
        let s = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let s = generator(s);

        assert_eq!(part2(&s), 2);
    }
}
