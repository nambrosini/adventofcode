use itertools::Itertools;

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<Vec<Cell>> {
    input.lines().map(|line| line.chars().map(|c| c.into()).collect_vec()).collect_vec()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Vec<Cell>]) -> usize {
    let mut input = input.to_vec();
    let mut history = vec![];

    loop {
        let score = calc_score(&input);
        if history.contains(&score) {
            return score;
        }
        history.push(score);
        input = simulate(&input);
    } 
}

pub fn simulate(map: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut new_map = map.to_vec();

    for (i, r) in map.iter().enumerate() {
        for (j, e) in r.iter().enumerate() {
            new_map[i][j] = match e {
                Cell::Alive => {
                    if count_neighbours(i, j, map) == 1 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                },
                Cell::Dead => {
                    let count = count_neighbours(i, j, map);
                    if count == 1 || count == 2 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
            };
        }
    }

    new_map
}

fn count_neighbours(x: usize, y: usize, map: &[Vec<Cell>]) -> usize {
    let neighbours = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    
    let x = x as i32;
    let y = y as i32;
    let len = map.len() as i32;

    let mut count = 0;

    for n in neighbours {
        if x + n.0 >= len || x + n.0 < 0 || y + n.1 >= len || y + n.1 < 0 {
            continue;
        }

        if map[(x + n.0) as usize][(y + n.1) as usize] == Cell::Alive {
            count += 1;
        }
    }

    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead
}

impl Cell {
    fn get_value(&self) -> usize {
        match self {
            Self::Alive => 1,
            Self::Dead => 0
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Dead,
            '#' => Cell::Alive,
            _ => unreachable!()
        }
    }
}

fn calc_score(map: &[Vec<Cell>]) -> usize {
    map.iter().flatten().enumerate().map(|(i, x)| x.get_value() * 2usize.pow(i as u32)).sum()
}

#[test]
fn test() {
    let s = ".....
.....
.....
#....
.#...";
    let s = generator(s);

    assert_eq!(calc_score(&s), 2129920);
}