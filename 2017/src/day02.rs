use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split('\t')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut checksum = 0u32;

    for l in input {
        checksum += l.iter().max().unwrap() - l.iter().min().unwrap();
    }

    checksum
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut checksum = 0u32;

    for l in input {
        let mut l = l.clone();
        l.sort_unstable();

        for i in 0..l.len() - 1 {
            for j in i + 1..l.len() {
                if l[j] % l[i] == 0 {
                    checksum += l[j] / l[i];
                }
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = input_generator("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8");

        assert_eq!(part1(&s), 18);
    }

    #[test]
    fn sample5() {
        let s = input_generator("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5\n");

        assert_eq!(part2(&s), 9);
    }
}
