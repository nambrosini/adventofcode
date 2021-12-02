use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<State>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    On,
    Off,
}

impl Grid {
    fn run(&mut self, step_count: usize, corners_on: bool) {
        for _ in 0..step_count {
            if corners_on {
                self.set_corners();
            }
            self.step();
        }

        if corners_on {
            self.set_corners();
        }
    }

    fn set_corners(&mut self) {
        let len = self.grid.len();
        for i in 0..=1 {
            for j in 0..=1 {
                self.grid[i * (len - 1)][j * (len - 1)] = State::On;
            }
        }
    }

    fn step(&mut self) {
        let mut grid_clone = self.grid.clone();
        for (i, r) in grid_clone.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                let count = self.check_neighbours(i, j);

                if self.grid[i][j] == State::On && count != 2 && count != 3 {
                    *e = State::Off;
                } else if count == 3 {
                    *e = State::On;
                }
            }
        }

        self.grid = grid_clone;
    }

    fn check_neighbours(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        let mut count = 0;
        for i in -1..=1 {
            if x + i < 0 || x + i >= self.grid.len() as i32 {
                continue;
            }
            for j in -1..=1 {
                if y + j < 0 || y + j >= self.grid.len() as i32 {
                    continue;
                }
                if i == 0 && j == 0 {
                    continue;
                }

                if self.grid[(x + i) as usize][(y + j) as usize] == State::On {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_on(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&x| x == State::On)
            .count()
    }
}

impl TryFrom<&str> for Grid {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let grid = value
            .lines()
            .map(|x| x.chars().map(|x| x.try_into().unwrap()).collect_vec())
            .collect_vec();

        Ok(Self { grid })
    }
}

impl TryFrom<char> for State {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(State::On),
            '.' => Ok(State::Off),
            _ => Err(format!("{} not recognized.", value)),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row {
                write!(
                    f,
                    "{}",
                    match cell {
                        State::On => '#',
                        State::Off => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::On => '#',
                State::Off => '.',
            }
        )
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Grid {
    input.try_into().unwrap()
}

#[aoc(day18, part1)]
pub fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.run(100, false);
    grid.count_on()
}

#[aoc(day18, part2)]
pub fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.run(100, true);
    grid.count_on()
}

#[test]
fn sample1_test1() {
    let s = std::fs::read_to_string("tests/day18/sample1.txt").unwrap();
    let mut grid = generator(&s);
    grid.run(4, false);
    assert_eq!(grid.count_on(), 4);
}

#[test]
fn sample1_test2() {
    let s = std::fs::read_to_string("tests/day18/sample1.txt").unwrap();
    let mut grid = generator(&s);
    grid.run(5, true);
    assert_eq!(grid.count_on(), 17);
}
