use std::fs;

use day05;

fn main() {
    let input = fs::read_to_string("input.in").unwrap();
    let mut memory: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut mem_clone = memory.clone();

    println!("Part 1: {}", day05::solve_part_1(&mut mem_clone, 1));

    println!("Part 2: {}", day05::solve_part_1(&mut memory, 5));
}
