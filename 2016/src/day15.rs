use std::collections::HashMap;
use regex::Regex;

#[aoc_generator(day15)]
fn generator(input: &str) -> Statue {
    let re = Regex::new(r"Disc #\d has (\d+) positions; at time=0, it is at position (\d+)\.").unwrap();

    let mut discs = Vec::new();

    for caps in re.captures_iter(input) {
        let positions = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let current = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

        discs.push(Disc::new(positions, current));
    }

    Statue::new(discs)
}

#[aoc(day15, part1)]
pub fn part1(statue: &Statue) -> usize {
    statue.get_passthrough_time()
}

#[aoc(day15, part2)]
pub fn part2(statue: &Statue) -> usize {
    let mut statue = statue.clone();

    statue.discs.push(Disc::new(11, 0));

    statue.get_passthrough_time()
}

#[derive(Debug, Copy, Clone)]
struct Disc {
    positions: usize,
    at_zero: usize
}

impl Disc {
    fn new(positions: usize, at_zero: usize) -> Self {
        Self {
            positions,
            at_zero
        }
    }

    fn calc_position_at(&self, time: usize) -> usize {
        (self.at_zero + time) % self.positions
    }
}

#[derive(Debug, Clone)]
pub struct Statue {
    discs: Vec<Disc>
}

impl Statue {
    fn new(discs: Vec<Disc>) -> Self {
        Self {
            discs
        }
    }

    fn get_passthrough_time(&self) -> usize {
        let mut time = 0;

        'outer: loop {
            let first_disc = self.discs[0].calc_position_at(time + 1);

            for i in 1..self.discs.len() {
                if self.discs[i].calc_position_at(time + i + 1) != first_disc {
                    time += 1;
                    continue 'outer;
                }
            }

            return time;
        }
    }
}

#[test]
fn test() {
    let s = Statue::new(
        vec![
            Disc::new(5, 4),
            Disc::new(2, 1)
        ]
    );

    assert_eq!(part1(&s), 5);
}