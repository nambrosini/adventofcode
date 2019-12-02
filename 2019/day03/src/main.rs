use std::fs;
use std::process;

use day03;

fn main() {
    let filename = "input.txt";

    let content = fs::read_to_string(filename).unwrap_or_else(|error| {
        eprintln!("Error reading file: {}", error);
        process::exit(1);
    });
}
