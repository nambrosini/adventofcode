use itertools::Itertools as _;
use std::collections::{HashSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    vertex: String,
    distance: usize
}

impl Edge {
    fn new(vertex: &str, distance: usize) -> Self {
        Self {
            vertex: vertex.to_owned(),
            distance
        }
    }
}

type MapType = HashMap<String, Vec<Edge>>;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> MapType {
    let mut map: MapType = HashMap::new();

    for l in input.lines() {
        let split = l.split(' ').collect_vec();
        let dist = split.last().unwrap().parse::<usize>().unwrap();
        let from = split[0].to_string();
        let to = split[2].to_string();

        let val = map.entry(from.clone()).or_default();
        val.push(Edge::new(&to, dist));

        let val = map.entry(to).or_default();
        val.push(Edge::new(&from, dist));
    }

    map
}

#[aoc(day9, part1)]
pub fn part1(map: &MapType) -> usize {
    let distances = calc_distances(map);

    *distances.iter().min().unwrap()
}

fn calc_distances(map: &MapType) -> HashSet<usize> {
    let mut distances = HashSet::new();
    let mut current_town: String;

    for (start, vertex) in map {
        for perm in vertex.iter().permutations(vertex.len()).unique() {
            current_town = start.to_owned();
            let mut sum = 0;
            for edge in perm {
                sum += map
                    .get(&current_town)
                    .unwrap()
                    .iter()
                    .filter(|e| e.vertex == edge.vertex)
                    .fold(0, |acc, e| acc + e.distance);
                current_town = edge.vertex.to_owned();
            }

            distances.insert(sum);
        }
    }
    distances
}

#[aoc(day9, part2)]
pub fn part2(map: &MapType) -> usize {
    let distances = calc_distances(map);

    *distances.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = generator(&std::fs::read_to_string("tests/day09/sample1.txt").unwrap());

        assert_eq!(part1(&input), 605);
    }
}