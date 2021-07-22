use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut s = input.to_owned();
    for _i in 0..40 {
        let v = parse(&s);
        s = String::new();
        for i in v {
            s.push_str(&i.len().to_string());
            s.push_str(&i[0..1]);
        }
    }

    s.len()
}

fn parse(s: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut i = 0;
    let chars = s.chars().collect_vec();

    while i < s.len() {
        let c = chars[i];
        let mut j = i + 1;
        while j < s.len() {
            if chars[j] != c {
                break;
            }
            j += 1;
        }

        res.push(chars[i..j].iter().collect());
        i = j;
    }

    res
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut s = input.to_owned();
    for _i in 0..50 {
        let v = parse(&s);
        s = String::new();
        for i in v {
            s.push_str(&i.len().to_string());
            s.push_str(&i[0..1]);
        }
    }

    s.len()
}