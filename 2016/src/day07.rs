use itertools::Itertools;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.to_string())
        .collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(input: &[String]) -> usize {
    let mut count = 0;

    for pass in input {
        let chars: Vec<char> = pass.chars().collect_vec();
        let mut in_brackets = false;
        let mut ok = false;
        for i in 0..chars.len() - 4 {
            if chars[i] == '[' || chars[i] == ']' {
                in_brackets = !in_brackets;
            }
            if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
                if in_brackets {
                    ok = false;
                    break;
                } else {
                    ok = true;
                }
            }
        }

        if ok {
            count += 1;
        }
    }

    count
}

#[test]
pub fn test1() {
    let s = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

    let s = generator(s);

    assert_eq!(part1(&s), 2);
}