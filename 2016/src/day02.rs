use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day02)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
}

#[aoc(day02, part1)]
pub fn part1(input: &[Vec<char>]) -> String {
    let mut grid = HashMap::new();

    for i in 0..3 as i32 {
        for j in 0..3 as i32 {
            grid.insert((i, j), (i * 3 + j + 1).to_string());
        }
    }

    get_res(input, &grid)
}

#[aoc(day02, part2)]
pub fn part2(input: &[Vec<char>]) -> String {
    let mut grid = HashMap::new();

    grid.insert((0, 2), "1".into());
    grid.insert((1, 1), "2".into());
    grid.insert((1, 2), "3".into());
    grid.insert((1, 3), "4".into());
    grid.insert((2, 0), "5".into());
    grid.insert((2, 1), "6".into());
    grid.insert((2, 2), "7".into());
    grid.insert((2, 3), "8".into());
    grid.insert((2, 4), "9".into());
    grid.insert((3, 1), "A".into());
    grid.insert((3, 2), "B".into());
    grid.insert((3, 3), "C".into());
    grid.insert((4, 2), "D".into());
    

    get_res(input, &grid)
}

fn get_res(input: &[Vec<char>], available_positions: &HashMap<(i32, i32), String>) -> String {
    let mut index: (i32, i32) = (1, 1);
    let mut res: String = String::new();

    for line in input {
        for i in line {
            match i {
                'U' => {
                    let new_index = (index.0 - 1, index.1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                },
                'D' => {
                    let new_index = (index.0 + 1, index.1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                },
                'R' => {
                    let new_index = (index.0, index.1 + 1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                },
                'L' => {
                    let new_index = (index.0, index.1 - 1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                },
                _ => unreachable!()
            }
        }
        res.push_str(&available_positions[&index]);
    }


    res
}

#[test]
fn test1() {
    let s = "ULL
RRDDD
LURDL
UUUUD";

    let s = generator(s);

    assert_eq!(part1(&s), "1985");
}