use itertools::Itertools;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<char> {
    input.chars().collect_vec()
}

#[aoc(day9, part1)]
pub fn part1(input: &[char]) -> i32 {
    let mut sum = 0;
    let mut _garbage_count = 0;
    let mut group_level = 0;
    let mut is_garbage = false;
    let mut is_canceled = false;

    for character in input {
        if is_canceled {
            is_canceled = false;
            continue;
        }
        if is_garbage {
            match character {
                '!' => {
                    is_canceled = true;
                }
                '>' => {
                    is_garbage = false;
                }
                _ => {
                    _garbage_count += 1;
                }
            }
            continue;
        }
        match character {
            '{' => {
                group_level += 1;
            }
            '}' => {
                sum += group_level;
                group_level -= 1;
            }
            '<' => {
                is_garbage = true;
            }
            _ => {}
        }
        continue;
    }

    sum
}

#[aoc(day9, part2)]
pub fn part2(input: &[char]) -> i32 {
    let mut _sum = 0;
    let mut garbage_count = 0;
    let mut group_level = 0;
    let mut is_garbage = false;
    let mut is_canceled = false;

    for character in input {
        if is_canceled {
            is_canceled = false;
            continue;
        }
        if is_garbage {
            match character {
                '!' => {
                    is_canceled = true;
                }
                '>' => {
                    is_garbage = false;
                }
                _ => {
                    garbage_count += 1;
                }
            }
            continue;
        }
        match character {
            '{' => {
                group_level += 1;
            }
            '}' => {
                _sum += group_level;
                group_level -= 1;
            }
            '<' => {
                is_garbage = true;
            }
            _ => {}
        }
        continue;
    }

    garbage_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let s1 = generator("{}");
        let s2 = generator("{{{}}}");
        let s3 = generator("{{},{}}");
        let s4 = generator("{{{},{},{{}}}}");
        let s5 = generator("{<a>,<a>,<a>,<a>}");
        let s6 = generator("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        let s7 = generator("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        let s8 = generator("{{<a!>},{<a!>},{<a!>},{<ab>}}");

        assert_eq!(part1(&s1), 1);
        assert_eq!(part1(&s2), 6);
        assert_eq!(part1(&s3), 5);
        assert_eq!(part1(&s4), 16);
        assert_eq!(part1(&s5), 1);
        assert_eq!(part1(&s6), 9);
        assert_eq!(part1(&s7), 9);
        assert_eq!(part1(&s8), 3);
    }
}
