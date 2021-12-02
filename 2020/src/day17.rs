use std::collections::{HashMap, HashSet};

type Coordinates = (i8, i8, i8, i8);

#[aoc_generator(day17)]
pub fn generator(input: &str) -> HashSet<Coordinates> {
    input
        .lines()
        .into_iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as i8, y as i8, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &HashSet<Coordinates>) -> usize {
    let mut s = input.clone();

    for _ in 0..6 {
        s = step(3, &s);
    }

    s.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &HashSet<Coordinates>) -> usize {
    let mut s = input.clone();

    for _ in 0..6 {
        s = step(4, &s);
    }

    s.len()
}

fn step(n: usize, s: &HashSet<Coordinates>) -> HashSet<Coordinates> {
    let mut m = HashMap::new();
    for (x, y, z, w) in s.iter() {
        for dx in if n > 0 { -1..=1 } else { 0..=0 } {
            for dy in if n > 1 { -1..=1 } else { 0..=0 } {
                for dz in if n > 2 { -1..=1 } else { 0..=0 } {
                    for dw in if n > 3 { -1..=1 } else { 0..=0 } {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        let p = (x + dx, y + dy, z + dz, w + dw);
                        m.insert(p, m.get(&p).unwrap_or(&0) + 1);
                    }
                }
            }
        }
    }
    m.iter()
        .filter_map(|(p, a)| {
            if *a == 3 || *a == 2 && s.contains(p) {
                Some(*p)
            } else {
                None
            }
        })
        .collect()
}


#[test]
fn sample1_test1() {
    let s = generator(".#.
..#
###");

    assert_eq!(part1(&s), 112);
}

#[test]
fn sample1_test2() {
    let s = generator(".#.
..#
###");

    assert_eq!(part2(&s), 848);
}
