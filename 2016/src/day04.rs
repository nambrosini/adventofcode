use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{From, Into};

#[aoc_generator(day04)]
pub fn generator(input: &str) -> Vec<Room> {
    input
        .lines()
        .map(|line| line.into())
        .collect_vec()
}

#[aoc(day04, part1)]
pub fn part1(input: &[Room]) -> usize {
    input
        .iter()
        .filter(|x| x.is_real())
        .map(|x| x.id)
        .sum()
}

#[aoc(day04, part2)]
pub fn part2(input: &[Room]) -> usize {
    let real = input
        .iter()
        .filter(|x| x.is_real())
        .collect_vec();

    let real = real.iter().map(|x| x.decrypt()).collect_vec();

    // let name = real
    //     .iter()
    //     .filter(|x| x.contains("north pole"))
    //     .cloned()
    //     .next()
    //     .unwrap();

    // input.iter().filter(|r| r.decrypt() == name).map(|r| r.id).next().unwrap()

    println!("{:?}", real);
    // real[0].clone()
    0

}

#[derive(Debug)]
pub struct Room {
    id: usize,
    name: String,
    map: HashMap<char, usize>,
    hash: String
}

impl Room {
    fn is_real(&self) -> bool {
        let mut letters = self.map
            .iter()
            .map(|(key, value)| (key, value))
            .collect_vec();

        letters.sort();

        let mut map: HashMap<String, usize> = HashMap::new();

        for i in self.map.values() {
            let x = letters.iter()
                .filter(|(_, &v)| v == *i)
                .sorted_by(|x, y| x.0.cmp(y.0))
                .map(|(k, _)| k)
                .join("");
            map.insert(x, *i);
        }

        let mut letters = map.iter().collect_vec();

        letters.sort_by_key(|(_, &value)| value);

        let mut s = String::new();

        for l in letters.iter().map(|x| x.0).rev() {
            let mut l = l.chars().collect_vec();
            l.sort_unstable();
            s.push_str(&l.iter().copied().collect::<String>());
        }

        s[0..5] == self.hash
    }

    fn decrypt(&self) -> String {
        let mut s = String::new();

        println!("{}", self.id);

        for c in self.name.chars() {
            if c == '-' {
                s.push(' ');
            } else {
                let new_c = (self.id as u8 % 26 + c as u8 - 97) % 26 + 97;
                s.push(new_c as char);
            }
        }

        s
    }
}

impl From<&str> for Room {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();
        let mut id = String::new();
        let mut is_hash = false;
        let mut hash = String::new();

        for c in s.chars() {
            if c == '[' {
                is_hash = true;
            } else if c == ']' {
                is_hash = false;
            } else if is_hash {
                hash.push(c);
            } else if c.is_digit(10) {
                id.push(c);
            } else if c != '-' {
                let e = map.entry(c).or_insert(0);
                *e += 1;
            }
        }

        let mut name = String::new();

        for c in s.chars() {
            if c.is_digit(10) {
                break;
            } else {
                name.push(c);
            }
        }

        Self { 
            id: id.parse().unwrap(),
            map,
            hash,
            name
        }
    }

}

// #[test]
// fn test1() {
//     let s = "aaaaa-bbb-z-y-x-123[abxyz]
// a-b-c-d-e-f-g-h-983[abcde]
// not-a-real-room-404[oarel]
// totally-real-room-200[decoy]";
//     let s = generator(s);

//     assert_eq!(part1(&s), 1514);
// }

// #[test]
// fn test2() {
//     let s = "north-pole-0[oehln]
// a-b-c-d-e-f-g-h-1[abcde]
// qzmt-zixmtkozy-ivhz-343[zimth]
// totally-real-room-200[decoy]
// aaaaa-bbb-z-y-x-123[abxyz]
// a-b-c-d-e-f-g-h-983[abcde]
// not-a-real-room-404[oarel]";
//     let s = generator(s);

//     assert_eq!(part2(&s), 1);
// }