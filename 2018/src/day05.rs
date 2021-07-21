use itertools::Itertools;
use std::collections::HashSet;
use std::vec;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<char> {
    input.chars().collect_vec()
}

#[aoc(day5, part1)]
pub fn part1(input: &[char]) -> usize {
    collapse(input).len()
}

#[aoc(day5, part2)]
pub fn part2(input: &[char]) -> usize {
    let elements: HashSet<char> = input
        .iter()
        .copied()
        .filter(|&c| c as usize >= 'a' as usize && c as usize <= 'z' as usize)
        .collect();

    let mut min = usize::MAX;

    for c in elements {
        let input = input
            .iter()
            .filter(|&&x| x != c && x as usize != c as usize - 32)
            .copied()
            .collect_vec();
        let collapsed_len = collapse(&input).len();

        if collapsed_len < min {
            min = collapsed_len;
        }
    }

    min
}

pub fn collapse(input: &[char]) -> Vec<char> {
    let mut input = input.to_vec();
    let mut destroyed = true;

    while destroyed {
        destroyed = false;

        let mut v = vec![];
        let mut count = 0;

        while count < input.len() - 1 {
            if (input[count] as i32 - input[count + 1] as i32).abs() != 32 {
                v.push(input[count]);
            } else {
                destroyed = true;
                count += 1;
            }

            count += 1;
        }

        if count == input.len() - 1 {
            v.push(input[count]);
        }

        input = v.clone();
    }

    input
}

#[test]
fn test1() {
    let s = "dabAcCaCBAcCcaDA";

    let s = generator(s);

    assert_eq!(part1(&s), 10);
}

#[test]
fn test2() {
    let s = "dabAcCaCBAcCcaDA";

    let s = generator(s);

    assert_eq!(part2(&s), 4);
}
