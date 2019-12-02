use std::fs;

use day02;

fn main() {
    let filename = "input.txt";
    let content: String = fs::read_to_string(filename).unwrap();
    let numbers: Vec<i32> = content
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut numbers_copy: Vec<i32> = Vec::from(&numbers[..]);
    println!(
        "Day2 part 1: {}",
        day02::execute_with_fixed_state(12, 2, &mut numbers_copy)
    );

    println!("Day2 part2: {}", day02::find_noun_and_verb(&numbers));
}
