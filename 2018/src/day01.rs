use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    vec![]
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = input_generator("1122");

        assert_eq!(part1(&s), 0);
    }
}
