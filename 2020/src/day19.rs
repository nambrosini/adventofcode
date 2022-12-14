use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day19)]
pub fn generator(input: &str) -> (HashMap<usize, String>, Vec<String>) {
    let mut lines = input.lines();

    let mut rules = HashMap::new();

    for line in &mut lines {
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
    compute_regex(raw_rules, &mut memo, 0);

    let expr = format!("^{}$", memo[0].as_ref().unwrap());

    let expr = Regex::new(&expr).unwrap();

    samples.iter().filter(|p| expr.is_match(p)).count()
}

#[aoc(day19, part2)]
pub fn part2((raw_rules, samples): &(HashMap<usize, String>, Vec<String>)) -> usize {
    let num_rules = raw_rules.keys().copied().max().unwrap() + 1;

    let mut memo = vec![None; num_rules];
    compute_regex(raw_rules, &mut memo, 0);

    // Rule 0 is 8 11
    // Rule 8 is 42+
    // Rule 11 is 42{n} 31{n} for arbtrary n
    // So we just need to know whether the sequence matches 42{n} 31{m} where n < m

    let r42 = memo[42].as_ref().unwrap();
    let r31 = memo[31].as_ref().unwrap();

    let r0 = format!("^({r42}+)({r31}+)$");

    let r0 = Regex::new(&r0).unwrap();
    let r42 = Regex::new(r42).unwrap();
    let r31 = Regex::new(r31).unwrap();

    samples
        .iter()
        .filter(|p| {
            if let Some(cap) = r0.captures(p) {
                let n = r42.find_iter(&cap[1]).count();
                let m = r31.find_iter(&cap[2]).count();

                n > m
            } else {
                false
            }
        })
        .count()
}

#[test]
fn sample1_test1() {
    let s = generator(
        "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb",
    );

    assert_eq!(part1(&s), 2);
}

#[test]
fn sample2_test2() {
    let s = generator(
        "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    );

    assert_eq!(part2(&s), 12);
}
