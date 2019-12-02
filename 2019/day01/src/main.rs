use std::fs;

use day01;

fn main() {
    let filename = "input.txt";
    let content = fs::read_to_string(filename).unwrap();

    let input: Vec<i32> = content.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("Day1 part1: {}", day01::calculate_weight(&input));
    println!("Day2 part2: {}", day01::calculate_fuel_weight(&input));
}
