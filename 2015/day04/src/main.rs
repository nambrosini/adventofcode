extern crate md5;

use std::fs;

use day04;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", day04::solve_part_1(&content));
    println!("Part 2: {}", day04::solve_part_2(&content));
}
