
use std::process;

use d02;

fn main() {
    let lines = d02::read_and_convert("input.txt").unwrap_or_else(|error| {
        eprintln!("Problem reading file: {}", error);
        process::exit(1);
    });

    let result = d02::calculate_fuel_weight(&lines);

    println!("{}", result);
}
