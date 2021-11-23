use std::fs;

use day13;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    day13::solve(&input);
}

