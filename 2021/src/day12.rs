use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> HashMap<String, Vec<String>> {
    let i = input
        .lines()
        .map(|line| line.split('-').collect::<Vec<&str>>())
        .map(|l| (l[0], l[1]))
        .collect_vec();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for l in i {
        if l.1 != "start" {
            let entry = map.entry(l.0.to_string()).or_default();
            entry.push(l.1.to_string());
        }

        if l.0 != "start" {
            let entry = map.entry(l.1.to_string()).or_default();
            entry.push(l.0.to_string());
        }
    }

    map
}

#[aoc(day12, part1)]
pub fn part1(input: &HashMap<String, Vec<String>>) -> usize {
    search(input, "start")
}

#[aoc(day12, part2)]
pub fn part2(input: &HashMap<String, Vec<String>>) -> usize {
    search2(input, "start")
}

fn visit(map: &HashMap<String, Vec<String>>, current: &str, path: &[String]) -> usize {
    if current == "end" {
        return 1;
    }

    let mut count = 0;

    let frontier = if let Some(frontier) = map.get(current) {
        frontier.clone()
    } else {
        vec![]
    };

    for next in frontier.iter() {
        if &next.to_lowercase() == next && path.contains(next) {
            continue;
        }
        let mut path = path.to_vec();
        path.push(current.to_string());
        count += visit(map, next, &path);
    }

    count
}

fn search(map: &HashMap<String, Vec<String>>, start: &str) -> usize {
    let new_path = vec![];

    visit(map, start, &new_path)
}

fn search2(map: &HashMap<String, Vec<String>>, start: &str) -> usize {
    let new_path = vec![];

    visit2(map, start, &new_path, false)
}

fn visit2(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    path: &[String],
    visited_twice: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut count = 0;

    let frontier = if let Some(frontier) = map.get(current) {
        frontier.clone()
    } else {
        vec![]
    };

    for next in frontier.iter() {
        if &next.to_lowercase() == next && path.contains(next) {
            if !visited_twice {
                let mut path = path.to_vec();
                path.push(current.to_string());
                count += visit2(map, next, &path, true);
            }
        } else {
            let mut path = path.to_vec();
            path.push(current.to_string());
            count += visit2(map, next, &path, visited_twice);
        }
    }

    count
}

#[test]
fn test() {
    let s = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    let s = generator(s);

    assert_eq!(part1(&s), 10);
}

#[test]
fn test1() {
    let s = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    let s = generator(s);

    assert_eq!(part1(&s), 226);
}

#[test]
fn test2() {
    let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let s = generator(s);

    assert_eq!(part1(&s), 19);
}

#[test]
fn test21() {
    let s = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    let s = generator(s);

    assert_eq!(part2(&s), 36);
}

#[test]
fn test22() {
    let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let s = generator(s);

    assert_eq!(part2(&s), 103);
}
