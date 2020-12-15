#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| l.to_owned())
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    let check_rule_1 = |s: &str| -> bool {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        s.chars()
            .filter(|x| vowels.contains(x))
            .count() >= 3
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

        not.iter()
            .filter(|x| s.contains(x.to_owned()))
            .count() == 0
    };

    input.iter()
        .filter(|x| check_rule_1(x) && check_rule_2(x) && check_rule_3(x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator("ugknbfddgicrmopn");

        assert_eq!(part1(&s), 1);
    }

    #[test]
    fn sample2_test1() {
        let s = generator("aaa");

        assert_eq!(part1(&s), 1);
    }

    #[test]
    fn sample3_test1() {
        let s = generator("jchzalrnumimnmhp");

        assert_eq!(part1(&s), 0);
    }

    #[test]
    fn sample4_test1() {
        let s = generator("haegwjzuvuyypxyu");

        assert_eq!(part1(&s), 0);
    }

    #[test]
    fn sample5_test1() {
        let s = generator("dvszwmarrgswjxmb");

        assert_eq!(part1(&s), 0);
    }
}