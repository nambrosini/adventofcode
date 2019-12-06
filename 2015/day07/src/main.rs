#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
}

enum Gate {
    And,
    RShift,
    LShift,
    Not,
    Or
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Gate::And),
            "RSHIFT" => Ok(Gate::RShift),
            "LSHIFT" => Ok(Gate::LShift),
            "NOT" => Ok(Gate::Not),
            "OR" => Ok(Gate::Or),
            _ => Err(format!("Unknown gate: {}", s))
        }
    }
}

struct Operation {
    input1_int: Option<u16>,
    input1_str: Option<String>,
    input2_int: Option<u16>,
    input2_str: Option<String>,
    output: String,
    gate: Gate
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<input1>[a-z]|\s)*(?P<gate>[A-Z]+|\s)*(?P<input2>[a-z]|\d)+ -> (?P<output>[a-z]+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let (input1_int, input1_str): (Option<u16>, Option<String>) = match caps.get(0).parse() {
            Ok(v) => (Some(v), None),
            Err(v) => (None, Some(caps["input1"].to_string()))
        };

        let (input1_str)
    }
}

