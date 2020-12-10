#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for i in input {
        if input.contains(&(2020 - i)) {
            return i * (2020 - i);
        }
    }

    unreachable!();
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for i in input {
        for j in input {
            if input.contains(&(2020 - i - j)) {
                return i * j * (2020 - i - j);
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn sample1_part1() {
        let one = input_generator(&fs::read_to_string("tests/day01/sample1").unwrap());

        assert_eq!(part1(&one), 514579);
    }

    #[test]
    fn sample1_part2() {
        let one = input_generator(&fs::read_to_string("tests/day01/sample1").unwrap());

        assert_eq!(part2(&one), 241861950);
    }
}