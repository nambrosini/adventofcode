use std::fs;

use day06;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", day06::solve_part_1(&input));
}

