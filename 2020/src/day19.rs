use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day19)]
pub fn generator_part1(input: &str) -> (HashMap<usize, String>, Vec<String>) {
    let mut lines = input.lines();

    let mut rules = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let pos = line.find(':').expect("Invalid line");

        let n = line[..pos].parse().unwrap();
        let remainder = line[(pos + 2)..].to_owned();

        rules.insert(n, remainder);
    }

    (rules, lines.map(|s| s.to_owned()).collect())
}

fn compute_regex(rules: &HashMap<usize, String>, memo: &mut [Option<String>], pos: usize) {
    if memo[pos].is_some() {
        return;
    }

    let rule = &rules[&pos];

    let mut expr = String::from("(?:");

    for part in rule.split(' ') {
        match part.chars().next().unwrap() {
            '|' => expr.push('|'),
            '"' => expr.push_str(&part[1..(part.len() - 1)]),
            c if c.is_ascii_digit() => {
                let i = part.parse().unwrap();
                compute_regex(rules, memo, i);
                expr.push_str(memo[i].as_ref().unwrap());
            }
            _ => panic!("Unknown regex bit: '{}'", part),
        }
    }

    expr.push(')');

    memo[pos] = Some(expr);
}

#[aoc(day19, part1)]
pub fn part1((raw_rules, samples): &(HashMap<usize, String>, Vec<String>)) -> usize {
    let num_rules = raw_rules.keys().copied().max().unwrap() + 1;

    let mut memo = vec![None; num_rules];
    compute_regex(&raw_rules, &mut memo, 0);

    let expr = format!("^{}$", memo[0].as_ref().unwrap());

    let expr = Regex::new(&expr).unwrap();

    samples.iter().filter(|p| expr.is_match(p)).count()
}

#[aoc(day19, part2)]
pub fn part2((raw_rules, samples): &(HashMap<usize, String>, Vec<String>)) -> usize {
    let num_rules = raw_rules.keys().copied().max().unwrap() + 1;

    let mut memo = vec![None; num_rules];
    compute_regex(&raw_rules, &mut memo, 0);

    // Rule 0 is 8 11
    // Rule 8 is 42+
    // Rule 11 is 42{n} 31{n} for arbtrary n
    // So we just need to know whether the sequence matches 42{n} 31{m} where n < m

    let r42 = memo[42].as_ref().unwrap();
    let r31 = memo[31].as_ref().unwrap();

    let r0 = format!("^({}+)({}+)$", r42, r31);

    let r0 = Regex::new(&r0).unwrap();
    let r42 = Regex::new(r42).unwrap();
    let r31 = Regex::new(r31).unwrap();

    samples
        .iter()
        .filter(|p| {
            if let Some(cap) = r0.captures(&p) {
                let n = r42.find_iter(&cap[1]).count();
                let m = r31.find_iter(&cap[2]).count();

                n > m
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator_part1(&std::fs::read_to_string("tests/day19/sample1").unwrap());

        assert_eq!(part1(&s), 2);
    }

    #[test]
    fn sample2_test2() {
        let s = generator_part1(&std::fs::read_to_string("tests/day19/sample2").unwrap());

        assert_eq!(part2(&s), 12);
    }
}
