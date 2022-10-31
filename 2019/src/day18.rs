use std::sync::mpsc;
use std::thread;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Vec<Cell>> {
    input.lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Cell>>())
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<Cell>]) -> usize {
    let input = input.to_vec();
    let start = find_entrance(&input);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        search_keys(&start, 0, &input, &[], tx)
    });

    rx.recv().unwrap()
}

fn search_keys(start: &(usize, usize), count: usize, maze: &[Vec<Cell>], keys: &[char], tx: std::sync::mpsc::Sender<usize>) {
    let found = find_point(start, maze, keys);
    let mut min_len = usize::MAX;

    let (tx1, rx1) = mpsc::channel();

    for (c, count, position) in found {
        let mut kk: Vec<char> = keys.to_vec();
        kk.push(c);
        let maze = maze.to_vec();
        let tx2 = tx1.clone();
        thread::spawn(move || {
            search_keys(&position, count, &maze, &kk, tx2);
        });
    }

    for r in rx1 {
        if r < min_len {
            min_len = r;
        }
    }

    tx.send(count + min_len).unwrap();
}

fn find_point(start: &(usize, usize), maze: &[Vec<Cell>], keys: &[char]) -> Vec<(char, usize, (usize, usize))> {
    let mut open_cells = vec![(*start, 0)];
    let mut close_cells = Vec::new();

    let mut found = vec![];

    while !open_cells.is_empty() {
        let (next_cell, count) = open_cells.remove(0);
        close_cells.push(next_cell);

        if &next_cell != start {
            if let Cell::Key(v) = maze[next_cell.0][next_cell.1] {
                if !keys.contains(&v) {
                    found.push((v, count, next_cell));
                }
            }
    
            if let Cell::Door(v) = maze[next_cell.0][next_cell.1] {
                if !keys.contains(&v.to_ascii_lowercase()) {
                    continue;
                }
            }
        }

        let frontier = expand_frontier(&next_cell, maze);

        for f in frontier.iter() {
            if close_cells.contains(f) {
                continue;
            }
            match maze[f.0][f.1] {
                Cell::Close => continue,
                _ => open_cells.push((*f, count + 1))
            }
        }
    }

    found
}

fn expand_frontier(frontier: &(usize, usize), maze: &[Vec<Cell>]) -> Vec<(usize, usize)> {
    let neighbours: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    let frontier = (frontier.0 as i32, frontier.1 as i32);
    let mut new_frontier = Vec::new();

    for n in neighbours {
        let x = frontier.0 + n.0;
        let y = frontier.1 + n.1;

        if x < 0 || x >= maze.len() as i32 || y < 0 || y >= maze[0].len() as i32 {
            continue;
        }

        new_frontier.push((x as usize, y as usize));
    }

    new_frontier
}

fn find_entrance(input: &[Vec<Cell>]) -> (usize, usize) {
    for (i, e) in input.iter().enumerate() {
        for (j, v) in e.iter().enumerate() {
            if v == &Cell::Entrance {
                return (i, j);
            }
        }
    }

    unreachable!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Close,
    Open,
    Door(char),
    Key(char),
    Entrance
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '#' { 
            Cell::Close
        } else if c == '.' {
            Cell::Open
        } else if c.is_lowercase() {
            Cell::Key(c)
        } else if c.is_uppercase() {
            Cell::Door(c)
        } else if c == '@' {
            Cell::Entrance
        } else {
            unreachable!()
        }
    }
}

#[test]
#[ignore]
fn test() {
    let s = "#########
#b.A.@.a#
#########";

    assert_eq!(part1(&generator(s)), 8);
}