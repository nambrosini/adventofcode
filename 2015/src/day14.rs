use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

#[derive(Debug, Clone)]
pub struct Race {
    reindeer: Reindeer,
    time: usize,
    km: usize,
}

impl TryFrom<&str> for Reindeer {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(' ').collect_vec();

        let name = split[0].to_owned();
        let speed = split[3].parse().unwrap();
        let fly_time = split[6].parse().unwrap();
        let rest_time = split[13].parse().unwrap();

        Ok(Self {
            name,
            speed,
            fly_time,
            rest_time,
        })
    }
}

impl Race {
    fn new(reindeer: Reindeer) -> Self {
        Self {
            reindeer,
            time: 0,
            km: 0,
        }
    }

    fn next_second(&mut self) -> usize {
        if self.time % (self.reindeer.fly_time + self.reindeer.rest_time) < self.reindeer.fly_time {
            self.km += self.reindeer.speed;
        }

        self.time += 1;
        self.km
    }

    fn complete_race(&mut self) -> usize {
        for _ in 0..2503 {
            self.next_second();
        }

        self.km
    }
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Race> {
    input
        .lines()
        .map(|l| Race::new(l.try_into().unwrap()))
        .collect_vec()
}

#[aoc(day14, part1)]
pub fn part1(map: &Vec<Race>) -> usize {
    let mut map = map.to_vec();
    map.iter_mut().map(|r| r.complete_race()).max().unwrap()
}

#[aoc(day14, part2)]
pub fn part2(map: &Vec<Race>) -> usize {
    let mut map = map.to_vec();
    let mut grid: HashMap<String, usize> = HashMap::new();

    for _ in 0..2503 {
        for x in map.iter_mut() {
            x.next_second();
        }

        let r = map.iter().max_by(|r0, r1| r0.km.cmp(&r1.km)).unwrap();

        let entry = grid.entry(r.reindeer.name.clone()).or_insert(0);
        *entry += 1;
    }

    *grid.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = std::fs::read_to_string("tests/day14/sample1.txt").unwrap();

        let generated = generator(&s);

        assert_eq!(part1(&generated), 2660);
    }

    #[test]
    fn sample1_test2() {
        let s = std::fs::read_to_string("tests/day14/sample1.txt").unwrap();

        let generated = generator(&s);

        assert_eq!(part2(&generated), 1564);
    }
}
