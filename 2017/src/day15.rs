use itertools::Itertools;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> (usize, usize) {
    let starts: Vec<usize> = input
        .lines()
        .map(|l| l.split(' ').last().unwrap().parse().unwrap())
        .collect_vec();

    (starts[0], starts[1])
}

#[aoc(day15, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    let mut generator_a = Generator::new(input.0, 16807);
    let mut generator_b = Generator::new(input.1, 48271);

    let mut count = 0;

    for _ in 0..40_000_000 {
        let a = generator_a.next().unwrap();
        let b = generator_b.next().unwrap();

        if a & 65_535 == b & 65_535 {
            count += 1;
        }
    }

    count
}

#[aoc(day15, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    let mut generator_a = SecondGenerator::new(input.0, 16807, 4);
    let mut generator_b = SecondGenerator::new(input.1, 48271, 8);

    let mut count = 0;

    for _ in 0..5_000_000 {
        let a = generator_a.next().unwrap();
        let b = generator_b.next().unwrap();

        if a & 65_535 == b & 65_535 {
            count += 1;
        }
    }

    count
}

#[derive(Debug, Copy, Clone)]
pub struct Generator {
    factor: usize,
    value: usize,
}

impl Generator {
    fn new(value: usize, factor: usize) -> Self {
        Generator { factor, value }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_val = (self.value * self.factor) % 2147483647;

        self.value = new_val;
        Some(new_val)
    }
}

pub struct SecondGenerator {
    value: usize,
    factor: usize,
    criteria: usize,
}

impl SecondGenerator {
    fn new(value: usize, factor: usize, criteria: usize) -> Self {
        Self {
            value,
            factor,
            criteria,
        }
    }
}

impl Iterator for SecondGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut new_val = (self.value * self.factor) % 2147483647;
        while new_val % self.criteria != 0 {
            self.value = new_val;
            new_val = (self.value * self.factor) % 2147483647;
        }
        self.value = new_val;

        Some(new_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1(&(65, 8921)), 588);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2(&(65, 8921)), 309);
    }
}
