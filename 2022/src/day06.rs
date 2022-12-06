use std::collections::HashSet;

#[aoc_generator(day06)]
pub fn generator(input: &str) -> Vec<char> {
    input.chars()
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[char]) -> usize {
    find_distinct_sequential(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &[char]) -> usize {
    find_distinct_sequential(input, 14)
}

fn find_distinct_sequential(input: &[char], size: usize) -> usize {
    for i in 0..input.len() - size {
        let set: HashSet<char> = input.iter().skip(i).take(size).copied().collect();
        if set.len() == size {
            return i + size;
        }
    }

    unreachable!()
}

#[test]
fn test() {
    let s = ["mjqjpqmgbljsphdztnvjfqwrcgsmlb", "bvwbjplbgvbhsrlpgdmjqwftvncz"];

    let want = [7, 5];

    for (i, w) in want.iter().enumerate() {
        assert_eq!(*w, part1(&generator(s[i])));
    }
}

#[test]
fn test1() {
    let s = ["mjqjpqmgbljsphdztnvjfqwrcgsmlb", "bvwbjplbgvbhsrlpgdmjqwftvncz"];

    let want = [19, 23];

    for (i, w) in want.iter().enumerate() {
        assert_eq!(*w, part2(&generator(s[i])));
    }
}