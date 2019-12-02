use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut sum = 0u32;

    for i in &lines {
        let n = i.parse::<u32>().unwrap();
        sum += n / 3 - 2;
    }

    println!("{}", sum);
}
