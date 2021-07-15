use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect_vec()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut sum = 0u32;

    for i in 0..input.len() {
        if input[i] == input[(i + 1) % input.len()] {
            sum += input[i]
        }
    }

    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut sum = 0u32;

    let halfway = input.len() / 2;

    for i in 0..input.len() {
        if input[i] == input[(i + halfway) % input.len()] {
            sum += input[i]
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = input_generator("1122");

        assert_eq!(part1(&s), 3);
    }

    #[test]
    fn sample2() {
        let s = input_generator("1111");

        assert_eq!(part1(&s), 4);
    }

    #[test]
    fn sample3() {
        let s = input_generator("1234");

        assert_eq!(part1(&s), 0);
    }

    #[test]
    fn sample4() {
        let s = input_generator("91212129");

        assert_eq!(part1(&s), 9);
    }

    #[test]
    fn sample5() {
        let s = input_generator("1212");

        assert_eq!(part2(&s), 6);
    }

    #[test]
    fn sample6() {
        let s = input_generator("1221");

        assert_eq!(part2(&s), 0);
    }

    #[test]
    fn sample7() {
        let s = input_generator("123425");

        assert_eq!(part2(&s), 4);
    }

    #[test]
    fn sample8() {
        let s = input_generator("123123");

        assert_eq!(part2(&s), 12);
    }
}
