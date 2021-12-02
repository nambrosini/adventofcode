use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> (Vec<char>, Vec<Config>) {
    let lines = input.lines().collect_vec();
    let initial: Vec<char> = lines[0].split(' ').collect_vec()[2].chars().collect_vec();
    let configs: Vec<Config> = lines[2..].iter()
        .map(|&line| line.into())
        .collect_vec();

    (initial, configs)
}

#[aoc(day12, part1)]
pub fn part1((initial, configs): &(Vec<char>, Vec<Config>)) -> i32 {
    let mut last: HashMap<i32, char> = HashMap::new();

    for i in 0..configs.len() as i32 {
        last.insert(i, initial[i as usize]);
    }

    for (i, e) in initial.iter().enumerate() {
        last.insert(i as i32, *e);
    }

    for _ in 0..20 {
        print(&last);
        let mut next_gen = HashMap::new();
        for i in last.keys() {
            let mut v = vec![];
            for j in i - 2..=i + 2 {
                if last.keys().any(|&k| k == j) {
                    v.push(last[&j]);
                } else {
                    v.push('.');
                    next_gen.insert(j, '.');
                }
            }
            next_gen.insert(*i, get_res_from_config(configs, &v));
        }
        last = next_gen;
    }

    last.iter()
        .filter(|(_, &v)| v == '#')
        .map(|(i, _)| *i)
        .sum()
}

fn print(map: &HashMap<i32, char>) {
    let min_key = *map.keys().min().unwrap();
    let max_key = *map.keys().max().unwrap();

    for i in min_key..=max_key {
        print!("{}", map[&i]);
    }
    println!();
}

fn get_res_from_config(configs: &[Config], val: &[char]) -> char {
    for c in configs.iter() {
        if c.equal(val) {
            return c.to;
        }
    }
    '.'
}

#[derive(Debug, Clone)]
pub struct Config {
    from: Vec<char>,
    to: char
}

impl Config {
    fn equal(&self, c: &[char]) -> bool {
        self.from == c
    }
}

impl From<&str> for Config {
    fn from(s: &str) -> Config {
        let mut split = s.split(" => ");

        let from = split.next().unwrap().chars().collect_vec();
        let to = split.next().unwrap().chars().next().unwrap();

        Config { 
            from,
            to
        }
    }
}

#[test]
fn test() {
    let s = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    assert_eq!(part1(&generator(s)), 324);
}