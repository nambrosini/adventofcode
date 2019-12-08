use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Part1: {}", solve_part_1(&input));
    solve_part_2(&input);
}

fn split(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let input = split(input);
    let input: Vec<Vec<i32>> = input.chunks(WIDTH * HEIGHT).map(|x| x.to_vec()).collect();

    let min = input
        .iter()
        .enumerate()
        .map(|(i, e)| (i, e.iter().filter(|&&x| x == 0).count()))
        .min_by_key(|(_, e)| *e)
        .unwrap();

    input[min.0].iter().filter(|&&x| x == 1).count()
        * input[min.0].iter().filter(|&&x| x == 2).count()
}

fn solve_part_2(input: &str) {
    let input = split(input);

    let mut pic = vec![];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let val = input
                .chunks(WIDTH * HEIGHT)
                .map(|layer| layer[(i * WIDTH) + j])
                .find(|&x| x != 2)
                .unwrap_or(2);

            pic.push(val)
        }
    }

    for line in pic.chunks(WIDTH) {
        for i in line {
            match i {
                1 => print!("▓"),
                _ => print!(" "),
            }
        }
        println!();
    }
}
