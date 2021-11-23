use itertools::Itertools;

#[aoc_generator(day01)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect_vec()
}

#[aoc(day01, part1)]
pub fn part1(input: &[usize]) -> usize {
    input.iter()
        .fold(0, |sum, x| sum + x / 3 - 2)
}

#[aoc(day01, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut sum: i32 = 0;

    for &i in input.iter() {
        let mut val: i32 = i as i32;

        loop {
            val = val / 3 - 2;
            if val > 0 {
                sum += val;
            } else {
                break;
            }
        }
    }

    sum as usize
}

#[test]
fn test() {
    let s = generator("14");

    assert_eq!(part1(&s), 2);
}

#[test]
fn test1() {
    let s = generator("1969");

    assert_eq!(part2(&s), 966);
}

#[test]
fn test2() {
    let s = generator("100756");

    assert_eq!(part2(&s), 50346);
}