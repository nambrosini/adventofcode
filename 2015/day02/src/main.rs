use std::fs;

use day02;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    println!("Part 1: {}", day02::solve_part_1(&lines));
}

