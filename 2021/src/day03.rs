use itertools::Itertools;

#[aoc_generator(day03)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day03, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..input[0].len() {
        let ones = input.iter().map(|l| l[i]).filter(|&x| x == 1).count();

        if ones > input.len() - ones {
            gamma += 1;
        } else {
            epsilon += 1;
        }
        if i < input[0].len() - 1 {
            epsilon <<= 1;
            gamma <<= 1;
        }
        println!("{:0b}", gamma);
    }

    gamma * epsilon
}

#[aoc(day03, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let mut o_two = input.to_vec();
    let mut co_two = input.to_vec();

    for i in 0..input[0].len() {
        let o_ones = o_two.iter().map(|l| l[i]).filter(|&x| x == 1).count();
        let co_ones = co_two.iter().map(|l| l[i]).filter(|&x| x == 1).count();

        if o_two.len() > 1 {
            if o_two.len() - o_ones <= o_ones && o_two.len() > 1 {
                o_two = o_two.iter().filter(|&x| x[i] == 1).cloned().collect();
            } else {
                o_two = o_two.iter().filter(|&x| x[i] == 0).cloned().collect();
            }
        }

        if co_two.len() == 1 {
            continue;
        }

        if co_two.len() - co_ones <= co_ones {
            co_two = co_two.iter().filter(|&x| x[i] == 0).cloned().collect();
        } else {
            co_two = co_two.iter().filter(|&x| x[i] == 1).cloned().collect();
        }
    }

    usize::from_str_radix(&o_two[0].iter().join(""), 2).unwrap()
        * usize::from_str_radix(&co_two[0].iter().join(""), 2).unwrap()
}

#[test]
fn test() {
    let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    assert_eq!(part1(&generator(s)), 198);
}

#[test]
fn test2() {
    let s = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    assert_eq!(part2(&generator(s)), 230);
}
