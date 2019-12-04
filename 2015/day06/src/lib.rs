#[macro_use] extern crate lazy_static;
extern crate regex;

use std::str::FromStr;
use std::num::ParseIntError;

use regex::Regex;

type Grid = Vec<Vec<bool>>;
type Point = (u32, u32);

pub fn solve_part_1(input: &str) -> u32 {
    let commands: Vec<Order> = input.lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut grid: Grid = vec![];

    for _ in 0..1000 {
        grid.push((0..1000).map(|_| false).collect());
    }

    for i in commands {
        for j in i.start.0..=i.end.0 {
            for k in i.start.1..=i.end.1 {
                grid[j as usize][k as usize] = match i.command {
                    Command::On => true,
                    Command::Off => false,
                    Command::Toggle => !grid[j as usize][k as usize]
                }
            }
        }
    }

    grid.into_iter().flatten().filter(|e| *e).count() as u32
}

#[derive(Debug)]
enum Command {
    Toggle,
    On,
    Off
}

#[derive(Debug)]
struct Order {
    command: Command,
    start: Point,
    end: Point
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Command::Toggle),
            "turn off" => Ok(Command::Off),
            "turn on" => Ok(Command::On),
            _ => {
                Err("Unknown command")
            }
        }
    }
}

impl FromStr for Order {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<command>([a-z]+|\s)+)(?P<start_x>\d+),(?P<start_y>\d+) ([a-z]+) (?P<end_x>\d+),(?P<end_y>\d+)").unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let command: Command = caps["command"].trim().parse().unwrap();
        let start_x = caps["start_x"].parse()?;
        let start_y = caps["start_y"].parse()?;
        let end_x = caps["end_x"].parse()?;
        let end_y = caps["end_y"].parse()?;

        Ok (Order{
            command, 
            start: (start_x, start_y), 
            end: (end_x, end_y)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_part_1("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(solve_part_1("toggle 0,0 through 999,0"), 1_000);
        assert_eq!(solve_part_1("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"), 999_996);
    }
}