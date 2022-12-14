use std::collections::HashMap;
use std::convert::{From, Into};

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Firewall {
    input
        .lines()
        .map(|l| {
            let depth = l.split(':').next().unwrap().parse().unwrap();
            (depth, l.into())
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Firewall) -> usize {
    input.iter().fold(0, |acc, x| {
        acc + if x.1.caught(0) { x.1.get_severity() } else { 0 }
    })
}

#[aoc(day13, part2)]
pub fn part2(input: &Firewall) -> usize {
    let mut delay = 0;
    let mut sum = usize::MAX;

    while sum > 0 {
        sum = input
            .iter()
            .fold(0, |acc, x| acc + usize::from(x.1.caught(delay)));

        delay += 1;
    }

    delay - 1
}

type Firewall = HashMap<usize, Layer>;

pub struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        Self { depth, range }
    }

    fn caught(&self, delay: usize) -> bool {
        let period = 2 * (self.range - 1);
        (self.depth + delay) % period == 0
    }

    fn get_severity(&self) -> usize {
        self.depth * self.range
    }
}

impl From<&str> for Layer {
    fn from(value: &str) -> Self {
        let mut split = value.split(": ");
        let depth = split.next().unwrap().parse().unwrap();
        let range = split.next().unwrap().parse().unwrap();
        Layer::new(depth, range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "0: 3
1: 2
4: 4
6: 4";
        let s = generator(s);

        assert_eq!(part1(&s), 24);
    }

    #[test]
    fn test2() {
        let s = "0: 3
1: 2
4: 4
6: 4";
        let s = generator(s);

        assert_eq!(part2(&s), 10);
    }
}
