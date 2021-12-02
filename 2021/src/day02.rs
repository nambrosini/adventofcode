#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Direction> {
    input.lines().map(|line| line.into()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Direction]) -> usize {
    let mut h = 0;
    let mut v = 0;

    for d in input {
        match d {
            Direction::Forward(k) => h += k,
            Direction::Up(k) => v -= k,
            Direction::Down(k) => v += k,
        }
    }

    h * v
}

#[aoc(day2, part2)]
pub fn part2(input: &[Direction]) -> usize {
    let mut aim = 0;
    let mut h = 0;
    let mut v = 0;

    for d in input {
        match d {
            Direction::Forward(k) => {
                h += k;
                v += aim * k;
            },
            Direction::Up(k) => aim -= k,
            Direction::Down(k) => aim += k,
        }
    }

    h * v
}

pub enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl From<&str> for Direction {
    fn from(d: &str) -> Direction {
        let split: Vec<&str> = d.split(" ").collect();
        let v: usize = split[1].parse().unwrap();

        match split[0] {
            "forward" => Direction::Forward(v),
            "up" => Direction::Up(v),
            "down" => Direction::Down(v),
            _ => unreachable!()
        }
    }
}

#[test]
fn test1() {
    let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    assert_eq!(part1(&generator(s)), 150);
}

#[test]
fn test2() {
    let s = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    assert_eq!(part2(&generator(s)), 900);
}