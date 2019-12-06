#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
}

enum Gate {
    And,
    Or,
    Not,
    RShift,
    LShift
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "NOT" => Ok(Gate::Not),
            "RShift" => Ok(Gate::RShift),
            "LShift" => Ok(Gate::LShift),
            _ => Err(format!("Unknown gate: {}", s))
        }
    }
}

struct Command {
    input1_int: Option<u16>,
    input1_str: Option<String>,
    input2_int: Option<u16>,
    input2_str: Option<String>,
    gate: Gate
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"");
        }
    }
}


