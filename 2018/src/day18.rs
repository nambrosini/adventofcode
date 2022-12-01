use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Grid {
    input.into()
}

#[aoc(day18, part1)]
pub fn part1(input: &Grid) -> usize {
    let mut grid = input.clone();
    for _ in 0..10 {
        grid.simulate_next_gen()
    }
    grid.calc_result()
}

#[aoc(day18, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut history: HashMap<u64, Vec<usize>> = HashMap::new();
    let minutes = 1_000_000_000;

    for minute in 0..minutes {
        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();

        let entry = history
            .entry(hash)
            .or_insert_with(Vec::new);
        entry.push(minute);

        if entry.len() == 5 {
            let period = entry[4] - entry[3];
            let remaining = minutes - minute;
            for _ in 0..remaining % period {
                grid.simulate_next_gen();
            }
            return grid.calc_result()
        }

        grid.simulate_next_gen();
    }
    grid.calc_result()
}

#[derive(Clone, Hash)]
pub struct Grid {
    grid: Vec<Vec<Acre>>
}

impl Grid {
    fn simulate_next_gen(&mut self) {
        let mut new_grid = self.grid.clone();

        for (i, r) in new_grid.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                let next = self.get_next(i, j);
                *e = next;
            }
        }

        self.grid = new_grid;
    }

    fn get_next(&self, x: usize, y: usize) -> Acre {
        let neig = self.get_neighbours_count(x, y);
        match self.grid[x][y] {
            Acre::Open => if neig.0 >= 3 {
                Acre::Tree
            } else {
                Acre::Open
            }
            Acre::Tree => if neig.1 >= 3 {
                Acre::Lumberyard
            } else {
                Acre::Tree
            }
            Acre::Lumberyard => if neig.0 >= 1 && neig.1 >= 1 {
                Acre::Lumberyard
            } else {
                Acre::Open
            }
        }
    }

    // Returns (trees, lumberyards)
    fn get_neighbours_count(&self, x: usize, y: usize) -> (usize, usize) {
        let mut count = (0, 0);
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if !(0..self.grid.len() as i32).contains(&(x as i32 + i)) ||
                    !(0..self.grid.len() as i32).contains(&(y as i32 + j)) {
                    continue;
                }

                let xi = (x as i32 + i) as usize;
                let yi = (y as i32 + j) as usize;

                if self.grid[xi][yi] == Acre::Tree {
                    count.0 += 1;
                } else if self.grid[xi][yi] == Acre::Lumberyard {
                    count.1 += 1;
                }
            }
        }

        count
    }

    fn calc_result(&self) -> usize {
        let trees_count = self.grid.iter()
            .flatten()
            .filter(|x| x == &&Acre::Tree)
            .count();
        let lumb_count = self.grid.iter()
            .flatten()
            .filter(|x| x == &&Acre::Lumberyard)
            .count();
        trees_count * lumb_count
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<Acre>> = s.lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();

        Self {
            grid
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Acre {
    Open = 0,
    Tree = 1,
    Lumberyard = 2
}

impl From<char> for Acre {
    fn from(s: char) -> Self {
        match s {
            '.' => Self::Open,
            '|' => Self::Tree,
            '#' => Self::Lumberyard,
            _ => unreachable!()
        }
    }
}