use std::fs;

use day03;
use day03::Wire;

fn main() {
    let filename = "input.txt";

    let content =  fs::read_to_string(filename).unwrap();

    let lines: Vec<Vec<&str>> = content.lines().map(|l| l.split(",").collect()).collect();

    let wire1 = Wire::new(&lines[0]);
    let wire2 = Wire::new(&lines[1]);

    println!("{}", wire1.manhattan(&wire2));
}

