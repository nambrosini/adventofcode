use std::fs;
use std::process;

use day01;

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).unwrap_or_else(|error| {
        eprintln!("Problem reading file: {}", error);
        process::exit(1);
    });

    let input: Vec<i32> = content
        .lines()
        .map(|x| {
            x.parse::<i32>().unwrap_or_else(|error| {
                eprintln!("Cannot convert number: {}", error);
                process::exit(1);
            })
        })
        .collect();

    println!("Day1 part1: {}", day01::calculate_weight(&input));
    println!("Day2 part2: {}", day01::calculate_fuel_weight(&input));
}
