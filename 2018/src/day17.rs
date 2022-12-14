use std::collections::HashMap;
use std::ops::Range;

type Ground = HashMap<(usize, usize), char>;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Ground {
    let mut ground: Ground = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(", ").collect();
        let (x, y) = if split[0].starts_with('x') {
            let x = get_range(&split[0][2..]);
            let y = get_range(&split[1][2..]);
            (x, y)
        } else {
            let y = get_range(&split[0][2..]);
            let x = get_range(&split[1][2..]);
            (x, y)
        };

        for i in x {
            for j in y.clone() {
                ground.insert((i, j), '#');
            }
        }
    }

    ground
}

fn get_range(s: &str) -> Range<usize> {
    let split: Vec<&str> = s.split("..").collect();
    if split.len() == 1 {
        let v = split[0].parse().unwrap();
        v..v
    } else {
        let v1 = split[0].parse().unwrap();
        let v2: usize = split[1].parse().unwrap();
        v1..v2 + 1
    }
}
