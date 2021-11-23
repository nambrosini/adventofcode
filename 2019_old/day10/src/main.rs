use std::fs;

use day10;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();

    println!("Part 1: {}", day10::solve_part_1(&input));

    println!("Part 2: {}", day10::solve_part_2(&input));
}
