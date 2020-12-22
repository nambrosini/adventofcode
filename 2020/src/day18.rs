use evalexpr::*;
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[String]) -> i64 {
    let mut sum: i64 = 0;
    for i in input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\((\d|\s|[+*])+\)").unwrap();
        }

        let mut s = i.clone();

        loop {
            let new_s = s.clone();
            let caps = RE.captures(&new_s);

            if let Some(v) = caps {
                let res = eval(v[0].to_owned());
                s = s.replace(&v[0], &res.to_string()).clone();
            } else {
                break;
            }
        }

        sum += eval(s);
    }

    sum
}

#[aoc(day18, part2)]
pub fn part2(input: &[String]) -> i64 {
    let mut sum: i64 = 0;
    for i in input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\((\d|\s|[+*])+\)").unwrap();
        }

        let mut s = i.clone();

        loop {
            let new_s = s.clone();
            let caps = RE.captures(&new_s);

            if let Some(v) = caps {
                let res = eval_part2(v[0].to_owned());
                s = s.replace(&v[0], &res.to_string()).clone();
            } else {
                break;
            }
        }

        sum += eval_part2(s);
    }

    sum
}

pub fn eval(expr: String) -> i64 {
    let expr = expr.replace("(", "").replace(")", "");
    let mut split: Vec<String> = expr.split(' ').map(|s| s.to_owned()).collect();

    while split.len() != 1 {
        let mut new: Vec<String> = vec![];

        let res = eval_int(&format!("{} {} {}", split[0], split[1], split[2])).unwrap();

        new.push(res.to_string());
        new.append(&mut split[3..].to_vec());
        split = new;
    }

    split[0].parse().unwrap()
}

pub fn eval_part2(expr: String) -> i64 {
    let expr = expr.replace("(", "").replace(")", "");
    let mut split: Vec<String> = expr.split(' ').map(|s| s.to_owned()).collect();

    while split.len() != 1 {
        let mut new: Vec<String> = vec![];

        if split.contains(&"*".to_owned()) && split.contains(&"+".to_owned()) {
            for i in (1..split.len() - 1).step_by(2) {
                if split[i] == "+" {
                    let res = eval_int(&format!("{} {} {}", split[i - 1], split[i], split[i + 1]))
                        .unwrap();
                    if i != 1 {
                        new = split[..i - 1].to_vec();
                    }
                    new.push(res.to_string());
                    new.append(&mut split[i + 2..].to_vec());
                    break;
                }
            }
        } else {
            let res = eval_int(&format!("{} {} {}", split[0], split[1], split[2])).unwrap();
            new.push(res.to_string());
            new.append(&mut split[3..].to_vec());
        }
        split = new;
    }

    split[0].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator("2 * 3 + (4 * 5)");

        assert_eq!(part1(&s), 26);
    }

    #[test]
    fn sample2_test1() {
        let s = generator("5 + (8 * 3 + 9 + 3 * 4 * 3)");

        assert_eq!(part1(&s), 437);
    }

    #[test]
    fn sample3_test1() {
        let s = generator("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

        assert_eq!(part1(&s), 12240);
    }

    #[test]
    fn sample4_test1() {
        let s = generator("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        assert_eq!(part1(&s), 13632);
    }

    #[test]
    fn sample5_test2() {
        let s = generator("1 + (2 * 3) + (4 * (5 + 6))");

        assert_eq!(part2(&s), 51);
    }

    #[test]
    fn sample6_test2() {
        let s = generator("2 * 3 + (4 * 5)");

        assert_eq!(part2(&s), 46);
    }

    #[test]
    fn sample7_test2() {
        let s = generator("5 + (8 * 3 + 9 + 3 * 4 * 3)");

        assert_eq!(part2(&s), 1445);
    }

    #[test]
    fn sample8_test2() {
        let s = generator("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

        assert_eq!(part2(&s), 669060);
    }

    #[test]
    fn sample9_test2() {
        let s = generator("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        assert_eq!(part2(&s), 23340);
    }
}
