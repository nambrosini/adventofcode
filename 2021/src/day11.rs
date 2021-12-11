use itertools::Itertools;
use std::collections::VecDeque;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect_vec()
}

#[aoc(day11, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    let mut map = input.to_vec();
    let mut flashes = 0;

    for _ in 0..100 {
        let mut queue = VecDeque::new();

        for (x, r) in map.iter_mut().enumerate() {
            for (y, e) in r.iter_mut().enumerate() {
                *e = (*e + 1) % 10;
                if *e == 0 {
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            flashes += 1;
            let neighbours = get_neighbours(&map, x, y);

            for (x, y) in neighbours {
                if map[x][y] == 0 {
                    continue;
                }
                map[x][y] = (map[x][y] + 1) % 10;
                if map[x][y] == 0 {
                    queue.push_back((x, y));
                }
            }
        }
    }

    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let mut map = input.to_vec();

    for i in 1.. {
        let mut queue = VecDeque::new();
        let mut flashes = 0;

        for (x, r) in map.iter_mut().enumerate() {
            for (y, e) in r.iter_mut().enumerate() {
                *e = (*e + 1) % 10;
                if *e == 0 {
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            flashes += 1;
            let neighbours = get_neighbours(&map, x, y);

            for (x, y) in neighbours {
                if map[x][y] == 0 {
                    continue;
                }
                map[x][y] = (map[x][y] + 1) % 10;
                if map[x][y] == 0 {
                    queue.push_back((x, y));
                }
            }
        }

        if flashes == map.len() * map[0].len() {
            return i;
        }
    }

    unreachable!()
}

fn get_neighbours(map: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    let xx = x as i32;
    let yy = y as i32;

    for i in -1i32..=1 {
        for j in -1i32..=1 {
            if i == 0 && j == 0
                || xx + i < 0
                || xx + i >= map.len() as i32
                || yy + j < 0
                || yy + j >= map[0].len() as i32
            {
                continue;
            }

            neighbours.push(((xx + i) as usize, (yy + j) as usize))
        }
    }

    neighbours
}

#[test]
fn test() {
    let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    assert_eq!(part1(&generator(s)), 1656);
}

#[test]
fn test2() {
    let s = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    assert_eq!(part2(&generator(s)), 195);
}