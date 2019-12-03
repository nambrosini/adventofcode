pub fn solve_part_1(input: &[&str]) -> u32 {
    input
        .iter()
        .filter(|s| !check_forbidden_words(s) && check_vowels(s) && check_double(s))
        .count() as u32
}

pub fn solve_part_2(input: &[&str]) -> u32 {
    input
        .iter()
        .filter(|s| check_twice_pair(s) && check_repeat_divided(s))
        .count() as u32
}

fn check_vowels(input: &str) -> bool {
    let vowels = vec!["a", "e", "i", "o", "u"];

    input
        .chars()
        .filter(|c| vowels.contains(&&c.to_string()[..]))
        .count()
        >= 3
}

fn check_forbidden_words(input: &str) -> bool {
    let forbidden_words = vec!["ab", "cd", "pq", "xy"];

    for i in forbidden_words {
        if input.contains(i) {
            return true;
        }
    }

    false
}

fn check_double(input: &str) -> bool {
    for i in 0..input.len() - 1 {
        let slice = &input[i..=i + 1];

        if slice[..1] == slice[1..] {
            return true;
        }
    }

    false
}

fn check_twice_pair(input: &str) -> bool {
    for i in 0..input.len() - 3 {
        for j in i + 2..input.len() - 1 {
            if input[i..=i + 1] == input[j..=j + 1] {
                return true;
            }
        }
    }

    false
}

fn check_repeat_divided(input: &str) -> bool {
    for i in 0..input.len() - 2 {
        if input[i..=i] == input[i + 2..=i + 2] {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec![
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ];

        assert_eq!(solve_part_1(&input), 2);
    }

    #[test]
    fn test_check_twice_pair() {
        let input = vec![
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ];

        assert_eq!(solve_part_2(&input), 2);
    }
}
