use std::ops::Add;

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut paths: Vec<State> = vec![State::new()];

    while !paths.is_empty() {
        let mut current_index = 0;
        let mut current_node = paths[0].clone();

        for (i, p) in paths.iter().skip(1).enumerate() {
            if p.path.len() < current_node.path.len() {
                current_node = p.clone();
                current_index = i;
            }
        }

        paths.remove(current_index);

        let doors = get_open_doors(input, &current_node.path);

        for door in doors {
            if let Some(new_state) = current_node.move_state(door) {
                if new_state.is_end_state() {
                    return new_state.path;
                }
                paths.push(new_state);
            }
        }
    }

    unreachable!();
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let state = State::new();

    let mut longest = 0;

    let doors = get_open_doors(input, &state.path);

    for door in doors {
        if let Some(new_state) = state.move_state(door) {
            longest = usize::max(longest, search(input, &new_state));
        }
    }

    longest
}

fn search(input: &str, state: &State) -> usize {
    if state.is_end_state() {
        return state.path.len();
    }

    let mut longest = 0;

    let doors = get_open_doors(input, &state.path);

    for door in doors {
        if let Some(new_state) = state.move_state(door) {
            longest = usize::max(longest, search(input, &new_state));
        }
    }

    longest
}

#[derive(Debug, Clone)]
struct State {
    position: Point,
    path: String,
}

impl State {
    fn new() -> State {
        State {
            position: Point::new(0, 0),
            path: String::new(),
        }
    }

    fn is_end_state(&self) -> bool {
        self.position == Point::new(3, 3)
    }

    fn move_state(&self, direction: Direction) -> Option<State> {
        let mut new_state = self.clone();
        new_state.position = new_state.position + direction.value();

        if new_state.position.x >= 0
            && new_state.position.x <= 3
            && new_state.position.y >= 0
            && new_state.position.y <= 3
        {
            new_state.path.push(direction.into());
            return Some(new_state);
        }

        None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

fn get_open_doors(input: &str, path: &str) -> Vec<Direction> {
    let open_chars = ['b', 'c', 'd', 'e', 'f'];
    let digest: Vec<char> = gen_digest(input, path).chars().take(4).collect();

    let mut directions = Vec::new();

    for (i, item) in digest.iter().enumerate() {
        if open_chars.contains(item) {
            let dir = match i {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => unreachable!(),
            };

            directions.push(dir);
        }
    }

    directions
}

fn gen_digest(input: &str, path: &str) -> String {
    let digest = md5::compute(format!("{}{}", input, path));
    format!("{:x}", digest)
}

#[test]
fn test11() {
    let s = "ihgpwlah";

    assert_eq!(part1(s), String::from("DDRRRD"));
}

#[test]
fn test12() {
    let s = "kglvqrro";

    assert_eq!(part1(s), String::from("DDUDRLRRUDRD"));
}

#[test]
fn test13() {
    let s = "ulqzkmiv";

    assert_eq!(part1(s), String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR"));
}

#[test]
fn test21() {
    let s = "ihgpwlah";

    assert_eq!(part2(s), 370);
}

#[test]
fn test22() {
    let s = "kglvqrro";

    assert_eq!(part2(s), 492);
}

#[test]
fn test23() {
    let s = "ulqzkmiv";

    assert_eq!(part2(s), 830);
}
