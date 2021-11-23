mod intcode;

use std::collections::HashMap;

use intcode::IntCode;
use intcode::color::Color;

pub fn solve(input: &str) {
    let input: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut memory: HashMap<i64, i64> = HashMap::new();

    for (i, &e) in input.iter().enumerate() {
        memory.insert(i as i64, e);
    }

    let mem_clone = memory.clone();

    let result = IntCode::new(memory).run(0).unwrap();

    println!("Part 1: {}", result.len() - 1);

    let result = IntCode::new(mem_clone).run(1).unwrap();

    let min_x = &result.keys().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = &result.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_x = &result.keys().max_by_key(|(x, _)| x).unwrap().0 - min_x + 1;
    let max_y = &result.keys().max_by_key(|(_, y)| y).unwrap().1 - min_y + 1;

    let mut grid: Vec<Vec<Color>> = vec![vec![Color::Black; max_x as usize]; max_y as usize];

    for ((x, y), c) in &result {
        grid[(y - min_y) as usize][(x - min_x) as usize] = c.clone();
    }

    println!("Part 2:");

    for i in grid {
        for j in i {
            match j {
                Color::Black => print!(" "),
                Color::White => print!("▓")
            }
        }
        println!();
    }
}

