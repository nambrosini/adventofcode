use std::collections::HashMap;

type Position = (usize, usize);
type Cave = HashMap<Position, char>;

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Cave {
    let mut cave = HashMap::new();

    for line in input.lines() {
        let split: Vec<(usize, usize)> = line.split(" -> ").map(|c| {
            let split: Vec<&str> = c.split(',').collect();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        }).collect();
        for i in 0..split.len() - 1 {
            let min_x = split[i].0.min(split[i + 1].0);
            let min_y = split[i].1.min(split[i + 1].1);
            let max_x = split[i].0.max(split[i + 1].0);
            let max_y = split[i].1.max(split[i + 1].1);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    cave.insert((x, y), '#');
                }
            }
        }
    }

    cave
}

#[aoc(day14, part1)]
pub fn part1(cave: &Cave) -> usize {
    let mut cave = cave.clone();
    let max_y = cave.keys().map(|p| p.1).max().unwrap();
    let mut count = 0;
    loop {
        let mut new_grain = (500, 0);

        while new_grain.1 <= max_y {
            if simulate_next(&mut cave, &mut new_grain) {
                count += 1;
                break;
            }
        }

        if new_grain.1 > max_y {
            return count;
        }
    }
}

#[aoc(day14, part2)]
pub fn part2(cave: &Cave) -> usize {
    let mut cave = add_floor(cave);
    let mut count = 0;
    loop {
        let mut new_grain = (500, 0);

        loop {
            if simulate_next(&mut cave, &mut new_grain) {
                count += 1;
                break;
            }
        }

        if new_grain == (500, 0) {
            return count;
        }
    }
}

/// Return true and adds the grain to the cave if it cannot move further, updates it position if it
/// can move.
fn simulate_next(cave: &mut Cave, new_grain: &mut (usize, usize)) -> bool {
    let next = (new_grain.0, new_grain.1 + 1);
    if cave.contains_key(&(new_grain.0, new_grain.1 + 1)) {
        if cave.get(&(next.0 - 1, next.1)).is_none() {
            new_grain.0 -= 1;
            return false;
        }
        if cave.get(&(next.0 + 1, next.1)).is_none() {
            new_grain.0 += 1;
            return false;
        }

        cave.insert(*new_grain, 'o');
        true
    } else {
        new_grain.1 += 1;
        false
    }
}

fn add_floor(cave: &Cave) -> Cave {
    let floor_y = cave.keys().map(|p| p.1).max().unwrap() + 2;
    let floor_start_x = 500 - floor_y;
    let floor_end_x = 500 + floor_y;

    let mut cave = cave.clone();

    for x in floor_start_x..=floor_end_x {
        cave.insert((x, floor_y), '#');
    }

    cave
}

#[test]
fn test() {
    let s = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(24, part1(&generator(s)));
}

#[test]
fn test2() {
    let s = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(93, part2(&generator(s)));
}