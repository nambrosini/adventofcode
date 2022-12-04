use std::ops::Range;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(Range<usize>, Range<usize>)> {
    input
        .lines()
        .map(|x| {
            let split: Vec<&str> = x.split(',').collect();
            let r1: Vec<&str> = split[0].split('-').collect();
            let r1 = r1[0].parse().unwrap()..r1[1].parse::<usize>().unwrap() + 1;
            let r2: Vec<&str> = split[1].split('-').collect();
            let r2 = r2[0].parse().unwrap()..r2[1].parse::<usize>().unwrap() + 1;
            (r1, r2)
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Range<usize>, Range<usize>)]) -> usize {
    input
        .iter()
        .filter(|(r1, r2)| r1.contains_range(r2))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Range<usize>, Range<usize>)]) -> usize {
    input.iter().filter(|(r1, r2)| r1.overlaps(r2)).count()
}

trait ContainsRange {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl ContainsRange for Range<usize> {
    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
            || other.start <= self.start && other.end >= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        for e in self.clone() {
            if other.contains(&e) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test1() {
    let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let res = part1(&generator(s));

    assert_eq!(2, res);
}

#[test]
fn test2() {
    let s = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    let res = part2(&generator(s));

    assert_eq!(4, res);
}
