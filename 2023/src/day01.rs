use std::fmt::format;
use itertools::Itertools;

#[aoc(day01, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calculate(l.to_string()))
        .sum()
}

#[aoc(day01, part2)]
pub fn part2(input: &str) -> u32 {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input.lines()
        .map(|l| {
            numbers_to_digits(numbers, l)
        })
        .map(calculate)
        .sum()
}

fn calculate(l: String) -> u32 {
    let it: Vec<char> = l.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    it[0].to_digit(10).unwrap() * 10 + it[it.len() - 1].to_digit(10).unwrap()
}

fn numbers_to_digits(numbers: [&str; 9], l: &str) -> String {
    let mut l = l.to_owned();
    let mut string = String::new();
    if l.len() < 3 {
        return l;
    }
    let mut i = 0;
    for (i, e) in l.chars().enumerate() {
        if e.is_ascii_digit() {
            string.push(e);
        }
        if i + 3 <= l.len() {
            let num = &l[i..i + 3];
            if numbers.contains(&num) {
                let n = numbers.iter().find_position(|x| x == &&num).unwrap().0 + 1;
                string.push_str(&format!("{}", n));
                continue;
            }
        }

        if i + 4 <= l.len() {
            let num = &l[i..i+4];
            if numbers.contains(&num) {
                let n = numbers.iter().find_position(|x| x == &&num).unwrap().0 + 1;
                string.push_str(&format!("{}", n));
            }
        }

        if i + 5 <= l.len() {
            let num = &l[i..i + 5];
            if numbers.contains(&num) {
                let n = numbers.iter().find_position(|x| x == &&num).unwrap().0 + 1;
                string.push_str(&format!("{}", n));
            }
        }
    }
    println!("{}", string);
    string
}

#[test]
fn test_1() {
    let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(part1(s), 142);
}

#[test]
fn test_2() {
    let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(s), 281);
}