#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|x| x.split("x").map(|x| x.parse().unwrap()).collect::<Vec<usize>>())
        .map(|x| {
            let mut x = x.clone();
            x.sort();
            x
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    input.iter()
        .map(|list| 2 * list[0] * list[1] + 2 * list[1] * list[2] + 2 * list[2] * list[0] + list[0] * list[1])
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    input.iter()
        .map(|l| 2 * l[0] + 2 * l[1] + l[0] * l[1] * l[2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator("2x3x4");

        assert_eq!(part1(&s), 58);
    }

    #[test]
    fn sample2_test1() {
        let s = generator("1x1x10");

        assert_eq!(part1(&s), 43);
    }

    #[test]
    fn sample1_test2() {
        let s = generator("2x3x4");

        assert_eq!(part2(&s), 34);
    }

    #[test]
    fn sample2_test2() {
        let s = generator("1x1x10");

        assert_eq!(part2(&s), 14);
    }
}