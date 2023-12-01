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
    let mut string = String::new();
    let mut i = 0;
    while i < l.len() {
        let c = l.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            string.push(c);
            i += 1;
            continue;
        }
        for x in 3..=5 {
            if i + x > l.len() {
                break;
            }

            let num = &l[i..i + x];
            if numbers.contains(&num) {
                let n = numbers.iter().find_position(|x| x == &&num).unwrap().0 + 1;
                string.push_str(&format!("{}", n));
                i += x - 2;
                break;
            }
        }
        i += 1;
    }
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