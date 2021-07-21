use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::fmt;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Message> {
    let mut input = input.lines().collect_vec();

    input.sort();

    input.iter().copied().map(|line| line.into()).collect_vec()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Message]) -> usize {
    let guards = parse_guards(input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().sum::<usize>())
        .unwrap();

    minutes_asleep.0
        * minutes_asleep
            .1
            .iter()
            .enumerate()
            .max_by_key(|x| x.1)
            .unwrap()
            .0
}

#[aoc(day4, part2)]
pub fn part2(input: &[Message]) -> usize {
    let guards = parse_guards(input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().max().unwrap())
        .unwrap();

    minutes_asleep.0
        * minutes_asleep
            .1
            .iter()
            .enumerate()
            .max_by_key(|x| x.1)
            .unwrap()
            .0
}

fn parse_guards(input: &[Message]) -> HashMap<usize, Guard> {
    let mut input = input.iter();
    let mut guards: HashMap<usize, Guard> = HashMap::new();

    let mut guard: Option<Guard> = None;

    while let Some(next) = input.next() {
        match next.action {
            Action::Begins => {
                let id = next.id.unwrap();
                if let Some(g) = guard.take() {
                    guards.insert(g.id, g);
                }

                guard = Some(guards.entry(id).or_insert(Guard::new(id)).clone());
            }
            _ => {
                if let Some(g) = guard.take() {
                    let mut g = g.clone();
                    g.update(next);
                    guard = Some(g);
                }
            }
        }
    }

    if let Some(g) = guard.take() {
        guards.insert(g.id, g);
    }

    guards
}

#[derive(Debug, Clone)]
struct Guard {
    id: usize,
    map: HashMap<(usize, usize), [bool; 60]>,
}

impl Guard {
    fn new(id: usize) -> Self {
        Self {
            id,
            map: HashMap::new(),
        }
    }

    fn update(&mut self, message: &Message) {
        let key = (message.day, message.month);
        match message.action {
            Action::Asleep => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in message.minute..60 {
                    e[m] = true;
                }
            }
            Action::Wakes => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in message.minute..60 {
                    e[m] = false;
                }
            }
            _ => {}
        }
    }

    fn get_freq_min(&self) -> Vec<usize> {
        let mut v: [usize; 60] = [0; 60];

        for (_, d) in self.map.iter() {
            for i in 0..d.len() {
                v[i] += d[i] as usize;
            }
        }

        v.to_vec()
    }
}

pub fn display(map: &HashMap<usize, Vec<usize>>) {
    println!("ID\tMinute");
    println!("\t000000000011111111112222222222333333333344444444445555555555");
    println!("\t012345678901234567890123456789012345678901234567890123456789");

    for (k, d) in map.iter() {
        print!("#{}\t", k);

        for i in d.iter() {
            let v = if i == &0 {
                String::from(".")
            } else if i >= &10 {
                String::from("9")
            } else {
                i.to_string()
            };
            print!("{}", v);
        }

        println!("\t{}", d.iter().sum::<usize>());
    }

    println!("")
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Date\tID\tMinute\n")?;
        write!(
            f,
            "\t\t000000000011111111112222222222333333333344444444445555555555\n"
        )?;
        write!(
            f,
            "\t\t012345678901234567890123456789012345678901234567890123456789\n"
        )?;

        let mut sorted_keys: Vec<(usize, usize)> = self.map.keys().copied().collect();
        sorted_keys.sort();

        for k in sorted_keys {
            write!(f, "{}-{}\t#{}\t", k.1, k.0, self.id)?;

            for i in self.map[&k].iter() {
                write!(f, "{}", if *i { "#" } else { "." })?;
            }

            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Begins,
    Asleep,
    Wakes,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        if s.contains("begins") {
            Action::Begins
        } else if s.contains("wakes") {
            Action::Wakes
        } else if s.contains("asleep") {
            Action::Asleep
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Message {
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    action: Action,
    id: Option<usize>,
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        let re =
            Regex::new(r"\[1518-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard #(\d+).+|.+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        let id = if let Some(x) = cap.get(6) {
            Some(x.as_str().parse::<usize>().unwrap())
        } else {
            None
        };

        Self {
            month: cap[1].parse().unwrap(),
            day: cap[2].parse().unwrap(),
            hour: cap[3].parse().unwrap(),
            minute: cap[4].parse().unwrap(),
            action: cap[5].into(),
            id,
        }
    }
}

#[test]
fn test1() {
    let s = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let s = generator(s);

    assert_eq!(part1(&s), 240);
}

#[test]
fn test2() {
    let s = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let s = generator(s);

    assert_eq!(part2(&s), 4455);
}
