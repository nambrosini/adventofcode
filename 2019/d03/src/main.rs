use std::fs;
use std::process;

use d03;

fn main() {
    let filename = "input.txt";

    let content: String = fs::read_to_string(filename).unwrap();
    let mut numbers: Vec<i32> = content.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    numbers[1] = 12;
    numbers[2] = 2;

    d03::run(&mut numbers);
}