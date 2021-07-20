use itertools::Itertools;

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |sum, x| sum + x)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut v = vec![];

    let mut sum = 0;
    let mut position = 0;

    while !v.contains(&sum) {
        v.push(sum);

        sum += input[position];

        position = (position + 1) % input.len();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s1 = generator("+1\n+1\n+1");
        let s2 = generator("+1\n+1\n-2");
        let s3 = generator("-1\n-2\n-3");

        assert_eq!(part1(&s1), 3);
        assert_eq!(part1(&s2), 0);
        assert_eq!(part1(&s3), -6);
    }
}
