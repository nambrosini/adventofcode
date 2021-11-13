use md5;
use std::collections::HashMap;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    solve_2(input, &gen_digest)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    solve_2(input, &gen_two_digest)
}

fn solve_2(input: &str, f: &dyn Fn(&str, usize) -> String) -> usize {
    let mut index = 0;
    let mut keys_count = 0;

    let mut map: HashMap<usize, char> = HashMap::new();

    loop {
        let digest = f(input, index);

        if index > 1000 {
            let index_to_remove: Vec<usize> = map
                .iter()
                .filter(|(&k, _)| k < index - 1000)
                .map(|(k, _)| *k)
                .collect();

            for i in index_to_remove {
                map.remove(&i);
            }
        }

        if let Some(c) = has_consecutive(&digest, 5) {
            let mut found_keys: Vec<usize> = map
                .iter()
                .filter(|(_, &v)| v == c)
                .map(|(k, _)| *k)
                .collect();
            found_keys.sort_unstable();

            for k in found_keys {
                keys_count += 1;

                if keys_count == 64 {
                    return k;
                }

                map.remove(&k);
            }
        }

        if let Some(c) = has_consecutive(&digest, 3) {
            map.insert(index, c);
        }

        index += 1;
    }
}

fn gen_digest(seed: &str, index: usize) -> String {
    let digest = md5::compute(format!("{}{}", seed, index));
    format!("{:x}", digest)
}

fn gen_two_digest(seed: &str, index: usize) -> String {
    let digest = md5::compute(format!("{}{}", seed, index));
    let mut digest = format!("{:x}", digest);

    for _ in 0..2016 {
        digest = format!("{:x}", md5::compute(digest));
    }

    digest
}

fn has_consecutive(s: &str, count: usize) -> Option<char> {
    let s: Vec<char> = s.chars().collect();

    for i in 0..=s.len() - count {
        let mut equals = true;
        for j in i + 1..i + count {
            if s[i] != s[j] {
                equals = false;
                break;
            }
        }

        if equals {
            return Some(s[i]);
        }
    }

    None
}

#[test]
fn test() {
    let res = part1("abc");

    assert_eq!(res, 22728);
}

#[test]
fn test_two() {
    let res = part2("abc");

    assert_eq!(res, 22551);
}

#[test]
fn test_three() {
    let s = "nasdlfansdcccasdf";

    assert_eq!(has_consecutive(s, 3), Some('c'));
}

#[test]
fn test_five() {
    let s = "asdfasdfhhhhhasdfasdfqwer";

    assert_eq!(has_consecutive(s, 5), Some('h'));
}

#[test]
fn test_five_two() {
    let s = "asdfasdfhhhhh";

    assert_eq!(has_consecutive(s, 5), Some('h'));
}
