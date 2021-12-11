use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect_vec()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for line in input {
        let mut stack = vec![];

        for &ch in line {
            if ch == ')' || ch == ']' || ch == '}' || ch == '>' {
                if let Some(c) = stack.pop() {
                    if get_other(&ch) != c {
                        sum += get_points(ch);
                    }
                }
            } else {
                stack.push(ch);
            }
        }
    }

    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let mut scores = vec![];
    for line in input {
        let mut stack = vec![];
        let mut ok = true;

        for &ch in line {
            if ch == ')' || ch == ']' || ch == '}' || ch == '>' {
                if let Some(c) = stack.pop() {
                    if get_other(&ch) != c {
                        ok = false;
                        break;
                    }
                }
            } else {
                stack.push(ch);
            }
        }

        if ok {
            let mut score = 0;
            for c in stack.iter().rev() {
                score = score * 5 + get_score(&get_other(c));
            }
            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn get_other(ch: &char) -> char {
    match ch {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn get_points(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn get_score(ch: &char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let s = generator(r"(]");
    assert_eq!(part1(&s), 57);
}

#[test]
fn test2() {
    let s = generator(r"{([(<{}[<>[]}>{[]{[(<()>");

    assert_eq!(part1(&s), 1197);
}

#[test]
fn test3() {
    let s = generator(r"[({(<(())[]>[[{[]{<()<>>");

    assert_eq!(part2(&s), 288957);
}
