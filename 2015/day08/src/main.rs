extern crate regex;

use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = input.replace("\n", "");

    let input = "\"\"\"abc\"\"aaa\\\"aaa\"\"\\x27\"";
    
    let total = input.len();
    let mut xs = 0i32;

    for i in input.chars() {
        if i == '"' {
            xs -= 1;
        }
    }

    for i in 0..input.len() - 1 {
        if &input[i..=i + 1] == "\"" {
            xs -= 1;
        } else if &input[i..=i + 1] == "\\x" {
            xs -= 3;
        }
    }

    println!("Total - memory: {} - {} = {}", total, xs.abs(), total as i32 + xs);
}
