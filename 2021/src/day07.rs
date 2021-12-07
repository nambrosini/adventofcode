use itertools::Itertools;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let max = *input.iter().max().unwrap();

    let mut min: i32 = i32::MAX;

    for i in 0..max {
        let sum = input.iter().map(|x| (x - i).abs()).sum::<i32>();
        if sum < min {
            min = sum;
        }
    }

    min
}

#[aoc(day07, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let max = *input.iter().max().unwrap();

    let mut min: i32 = i32::MAX;

    for i in 0..max {
        let sum = input.iter().map(|x| sum_prev((x - i).abs())).sum::<i32>();
        if sum < min {
            min = sum;
        }
    }

    min
}

fn sum_prev(x: i32) -> i32 {
    (x / 2) * (1 + x)
}