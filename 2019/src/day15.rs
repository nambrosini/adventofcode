use std::{collections::{HashMap, HashSet}, ops::Add, hash::Hash};

#[aoc_generator(day15)]
pub fn generator(input: &str) -> HashMap<Position, i64> {
    let mem: Vec<i64> = input.split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map: HashMap<Position, i64> = HashMap::new();
    map.insert(Position::new(0, 0), 0);

    move_robot(&mut map, &Intcode::new(&mem), &Position::new(0, 0));

    map
}

#[aoc(day15, part1)]
pub fn part1(map: &HashMap<Position, i64>) -> i64 {
    print_map(map);

    let start = Position::new(0, 0);

    search(map, &start)
}

#[aoc(day15, part2)]
pub fn part2(map: &HashMap<Position, i64>) -> i64 {
    let start = *map.iter()
        .find(|(_, v)| v == &&2)
        .unwrap()
        .0;

    fill_oxygen(map, &start)
}

pub fn fill_oxygen(map: &HashMap<Position, i64>, start: &Position) -> i64 {
    let mut queue = vec![(*start, 0)];
    let mut set = HashSet::new();

    while let Some(p) = queue.pop() {
        for i in 1..=4 {
            let new_pos = p.0 + <i64 as std::convert::Into<Direction>>::into(i).into();
            if map.contains_key(&new_pos) && !set.iter().any(|(p, _)| p == &new_pos) {
                let v = map[&new_pos];
                if v != 0 {
                    set.insert((new_pos, p.1 + 1));
                    queue.push((new_pos, p.1 + 1));
                }
            }
        }
    }

    set.iter()
        .max_by_key(|x| x.1)
        .unwrap()
        .1
}

pub fn search(map: &HashMap<Position, i64>, start: &Position) -> i64 {
    let mut queue = vec![(*start, 0)];
    let mut set = HashSet::new();

    while let Some(p) = queue.pop() {
        if map[&p.0] == 2 {
            return p.1;
        }

        for i in 1..=4 {
            let new_pos = p.0 + <i64 as std::convert::Into<Direction>>::into(i).into();
            if map.contains_key(&new_pos) && !set.contains(&new_pos) {
                let v = map[&new_pos];
                if v != 0 {
                    set.insert(new_pos);
                    queue.push((new_pos, p.1 + 1));
                }
            }
        }
    }

    unreachable!()
}

pub fn move_robot(map: &mut HashMap<Position, i64>, pc: &Intcode, pos: &Position) {
    for i in 1..=4i64 {
        let new_pos = *pos + (<i64 as std::convert::Into<Direction>>::into(i).into());
        if !map.keys().any(|p| p == &new_pos) {
            let mut pc_clone = pc.clone();
            let ret = pc_clone.run(i).unwrap();
            map.insert(new_pos, ret);

            if ret != 0 {
                move_robot(map, &pc_clone, &new_pos);
            }
        }
    }
}

#[derive(Clone)]
pub struct Intcode {
    mem: HashMap<usize, i64>,
    pos: usize,
    rel_base: usize
}

impl Intcode {
    fn new(mem: &[i64]) -> Self {
        Self {
            mem: mem.iter().copied().enumerate().collect(),
            pos: 0,
            rel_base: 0
        }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        loop {
            let op: Opcode = self.mem[&self.pos].into();
            match op {
                Opcode::Add(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);
    
                    self.set_mem(3, v1 + v2, m3);
                    self.pos += 4;
                },
                Opcode::Mul(m1, m2, m3) => {
                    let v1 = self.get_mem(1, m1);
                    let v2 = self.get_mem(2, m2);
    
                    self.set_mem(3, v1 * v2, m3);
                    self.pos += 4;
                },
                Opcode::Save(m1) => {
                    self.set_mem(1, input, m1);
                    self.pos += 2;
                },
                Opcode::Out(m1) => {
                    let res = self.get_mem(1, m1);
                    self.pos += 2;
                    return Some(res);
                },
                Opcode::Jit(m1, m2) => {
                    if self.get_mem(1, m1) != 0 {
                        self.pos = self.get_mem(2, m2) as usize;
                    } else {
                        self.pos += 3;
                    }
                },
                Opcode::Jif(m1, m2) => {
                    if self.get_mem(1, m1) == 0 {
                        self.pos = self.get_mem(2, m2) as usize;
                    } else {
                        self.pos += 3;
                    }
                },
                Opcode::Lt(m1, m2, m3) => {
                    let v = if self.get_mem(1, m1) < self.get_mem(2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set_mem(3, v, m3);
                    self.pos += 4;
                },
                Opcode::Eq(m1, m2, m3) => {
                    let v = if self.get_mem(1, m1) == self.get_mem(2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set_mem(3, v, m3);
                    self.pos += 4;
                },
                Opcode::Rb(m1) => {
                    self.rel_base += self.get_mem(1, m1) as usize;
                    self.pos += 2;
                },
                Opcode::Exit => {
                    return None
                }
            }
        }
    }

    fn set_mem(&mut self, offset: usize, val: i64, mode: Mode) {
        let index = self.get_index(offset, mode);
        let entry = self.mem.entry(index).or_insert(0);
        *entry = val;
    }

    fn get_mem(&self, offset: usize, mode: Mode) -> i64 {
        let index = self.get_index(offset, mode);
        self.mem[&index]
    }

    fn get_index(&self, offset: usize, mode: Mode) -> usize {
        match mode {
            Mode::Pos => self.mem[&(self.pos + offset)] as usize,
            Mode::Imm => self.pos + offset,
            Mode::Rel => self.mem[&(self.pos + offset)] as usize + self.rel_base,
        }
    }
}

#[derive(Debug)]
enum Mode {
    Pos,
    Imm,
    Rel
}

impl From<i64> for Mode {
    fn from(x: i64) -> Self {
        match x {
            0 => Self::Pos,
            1 => Self::Imm,
            2 => Self::Rel,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Eq(Mode, Mode, Mode),
    Rb(Mode),
    Exit
}

impl From<i64> for Opcode {
    fn from(x: i64) -> Self {
        let op = x % 100;
        let m1: Mode = (x / 100 % 10).into();
        let m2: Mode = (x / 1000 % 10).into();
        let m3: Mode = (x / 10000 % 10).into();

        match op {
            1 => Self::Add(m1, m2, m3),
            2 => Self::Mul(m1, m2, m3),
            3 => Self::Save(m1),
            4 => Self::Out(m1),
            5 => Self::Jit(m1, m2),
            6 => Self::Jif(m1, m2),
            7 => Self::Lt(m1, m2, m3),
            8 => Self::Eq(m1, m2, m3),
            9 => Self::Rb(m1),
            99 => Self::Exit,
            _ => unreachable!()
        }
    }
}

pub enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4
}

impl From<i64> for Direction {
    fn from(x: i64) -> Self {
        match x {
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Left,
            4 => Direction::Right,
            _ => unreachable!()
        }
    }
}

impl From<Direction> for Position {
    fn from(s: Direction) -> Self {
        match s {
            Direction::Up => Position::new(0, -1),
            Direction::Down => Position::new(0, 1),
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn print_map(map: &HashMap<Position, i64>) {
    let min_x = map.keys().min_by_key(|k| k.x).unwrap().x;
    let max_x = map.keys().max_by_key(|k| k.x).unwrap().x;
    let min_y = map.keys().min_by_key(|k| k.y).unwrap().y;
    let max_y = map.keys().max_by_key(|k| k.y).unwrap().y;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let v = if let Some(v) = map.get(&Position { x, y }) {
                if x == 0 && y == 0{
                    "S"
                } else {
                    match v {
                        0 => "▓",
                        1 => ".",
                        2 => "X",
                        _ => unreachable!()
                    }
                }
            } else {
                "▓"
            };
            print!("{}", v);
        }
        println!()
    }
}

#[test]
fn test() {
    let s = std::fs::read_to_string("input/2019/day15.txt").unwrap();

    assert_eq!(308, part1(&generator(&s)));
}