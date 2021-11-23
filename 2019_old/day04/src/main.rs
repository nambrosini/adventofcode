use std::fs;

use day04;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let start: u32 = input[..6].parse().unwrap();
    let end: u32 = input[7..].parse().unwrap();

    println!("Part 1: {}", day04::solve_part_1(start, end));
    println!("Part 2: {}", day04::solve_part_2(start, end));
}
