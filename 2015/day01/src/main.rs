use std::fs;

use day01;

fn main() {
    let filename = "input.txt";

    let content = fs::read_to_string(filename).unwrap();

    println!("Part1: {}", day01::solve_part_1(&content));
    println!("Part2: {}", day01::solve_part_2(&content));
}