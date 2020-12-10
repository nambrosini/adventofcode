#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut input: Vec<u32> = input.lines()
        .map(|x| x.parse().unwrap())
        .collect();

    input.sort();

    input
}

#[aoc(day10, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let differences: Vec<u32> = input
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();

    let diff1: u32 = differences.iter().filter(|&&x| x == 1u32).count() as u32;
    let diff3: u32 = differences.iter().filter(|&&x| x == 3u32).count() as u32;

    match input[0] - 0 {
        1 => ((diff1 + 1) * (diff3 + 1)),
        3 => (diff1 * (diff3 + 2)),
        _ => panic!()
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[u32]) -> u64 {
    let max = *input.last().unwrap() as usize;
    let mut combinations: Vec<u64> = vec![0; max + 3usize];
    combinations[max] = 1;
    for rating in input.iter().rev().skip(1) {
        for i in 1..=3 {
            combinations[*rating as usize] += combinations[(rating + i) as usize];
        }
    }
    for i in 1..=3 {
        combinations[0] += combinations[i];
    }

    combinations[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn sample1_part1() {
        let one = input_generator(&fs::read_to_string("tests/day10/sample1").unwrap());

        assert_eq!(part1(&one), 35);
    }

    #[test]
    fn sample1_part2() {
        let one = input_generator(&fs::read_to_string("tests/day10/sample1").unwrap());

        assert_eq!(part2(&one), 8);
    }

    #[test]
    fn sample2_part1() {
        let one = input_generator(&fs::read_to_string("tests/day10/sample2").unwrap());

        assert_eq!(part1(&one), 220);
    }

    #[test]
    fn sample2_part2() {
        let one = input_generator(&fs::read_to_string("tests/day10/sample2").unwrap());

        assert_eq!(part2(&one), 19208);
    }
}