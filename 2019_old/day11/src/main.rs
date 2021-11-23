use std::fs;

use day11;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    day11::solve(&input);
}
