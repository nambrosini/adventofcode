use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let split = input.split("\n\n").collect_vec();

    let coords: HashSet<Point> = split[0].lines()
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .map(|l| (l[0].parse().unwrap(), l[1].parse().unwrap()))
        .collect();

    let folds = split[1].lines()
        .map(|l| {
            let s: Vec<&str> = l.split(' ').last().unwrap().split('=').collect();
            let axis = match s[0] {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!()
            };
            let val = s[1].parse().unwrap();

            (axis, val)
        })
        .collect_vec();

    (coords, folds)
}

#[aoc(day13, part1)]
pub fn part1((coords, folds): &(HashSet<Point>, Vec<Fold>)) -> usize {
    let mut coords: HashSet<Point> = coords.clone();

    coords = fold(&coords, &folds[0]);

    coords.len()
}

#[aoc(day13, part2)]
pub fn part2((coords, folds): &(HashSet<Point>, Vec<Fold>)) -> String {
    let mut coords: HashSet<Point> = coords.clone();

    for f in folds.iter() {
        coords = fold(&coords, f);
    }

    print(&coords)
}

fn fold(map: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    match fold.0 {
        Axis::X => {
            let line = fold.1;
            let mut new_map = HashSet::new();

            for p in map {
                if p.0 > line {
                    let new_p = (
                        line - (p.0 - line), 
                        p.1
                    );
                    new_map.insert(new_p);
                } else {
                    new_map.insert(*p);
                }
            }
            new_map
        },
        Axis::Y => {
            let line = fold.1;
            let mut new_map = HashSet::new();

            for p in map {
                if p.1 > line {
                    let new_p = (
                        p.0,
                        line - (p.1 - line),
                    );
                    new_map.insert(new_p);
                } else {
                    new_map.insert(*p);
                }
            }
            new_map
        }
    }
}

type Point = (usize, usize);
type Fold = (Axis, usize);

pub enum Axis {
    X,
    Y
}

fn print(coords: &HashSet<Point>) -> String {
    let max_x = coords.iter().max_by_key(|c| c.1).unwrap().1;
    let max_y = coords.iter().max_by_key(|c| c.0).unwrap().0;

    let mut s = String::from("\n");

    for j in 0..=max_x {
        for i in 0..=max_y {
            if coords.contains(&(i, j)) {
                s.push('█');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    
    s
}

#[test]
fn test() {
    let s = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let s = generator(s);

    assert_eq!(part1(&s), 17);
}