use std::fs;

use day06;

fn main() {
    let orbits: String = fs::read_to_string("input.in").unwrap();
    let (p1, p2) = day06::solve_part_1(&orbits);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
