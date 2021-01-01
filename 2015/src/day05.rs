#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    let check_rule_1 = |s: &str| -> bool {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        s.chars().filter(|x| vowels.contains(x)).count() >= 3
    };

    let check_rule_2 = |s: &str| -> bool {
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                return true;
            }
        }

        false
    };

    let check_rule_3 = |s: &str| -> bool {
        let not = vec!["ab", "cd", "pq", "xy"];

        not.iter().filter(|x| s.contains(x.to_owned())).count() == 0
    };

    input
        .iter()
        .filter(|x| check_rule_1(x) && check_rule_2(x) && check_rule_3(x))
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[String]) -> usize {
    let check_rule_1 = |s: &str| -> bool {
        for i in 0..s.len() - 2 {
            let pair = &s[i..=i + 1];
            if s[..=i + 2].contains(pair) && s[i + 2..].contains(pair) {
                return true;
            }
        }

        false
    };

    let check_rule_2 = |s: &str| -> bool {
        for i in 0..s.len() - 2 {
            if s[i..=i] == s[i + 2..=i + 2] {
                return true;
            }
        }

        false
    };

    input
        .iter()
        .filter(|x| check_rule_1(x.trim()) && check_rule_2(x.trim()))
        .count()
}
