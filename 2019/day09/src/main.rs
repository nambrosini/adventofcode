use std::fs;

use day09::IntCode;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let memory: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mem_clone = memory.clone();

    println!("Part 1: {}", IntCode::new(mem_clone).run(1).unwrap());

    println!("Part 2: {}", IntCode::new(memory).run(2).unwrap());
}
