use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    input.windows(2).filter(|x| x[1] > x[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|x| x.iter().sum::<usize>())
        .collect_vec()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
}
