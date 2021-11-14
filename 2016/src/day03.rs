use itertools::Itertools;

#[aoc_generator(day03)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.split('\t').map(|x| x.parse().unwrap()).collect_vec())
        .collect_vec()
}

#[aoc(day03, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|x| is_triangle(x)).count()
}

#[aoc(day03, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut sum = 0;

    for i in 0..3 {
        let vec: Vec<usize> = input
            .iter()
            .flatten()
            .skip(i)
            .enumerate()
            .filter(|(i, _)| i % 3 == 0)
            .map(|(_, &e)| e)
            .collect_vec();

        sum += vec.chunks(3).filter(|x| is_triangle(x)).count()
    }

    sum
}

pub fn is_triangle(triangle: &[usize]) -> bool {
    let mut triangle = triangle.to_vec();

    triangle.sort_unstable();

    triangle[0] + triangle[1] > triangle[2]
}
