use std::fs;

use day05;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = content.lines().collect();

    println!("Part 1: {}", day05::solve_part_1(&lines));
    println!("Part 2: {}", day05::solve_part_2(&lines));
}
