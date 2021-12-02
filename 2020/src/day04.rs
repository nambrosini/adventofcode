use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl TryFrom<&str> for Passport {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(\w+):([^\s]+)\s*").unwrap();

        let mut map = HashMap::new();
        re.captures_iter(val).fold((), |_, v| {
            map.insert(String::from(&v[1]), String::from(&v[2]));
        });

        let mut passport = Passport::default();

        for entry in map.iter() {
            match &(entry.0)[..] {
                "byr" => passport.byr = Some(String::from(entry.1)),
                "iyr" => passport.iyr = Some(String::from(entry.1)),
                "eyr" => passport.eyr = Some(String::from(entry.1)),
                "hgt" => passport.hgt = Some(String::from(entry.1)),
                "hcl" => passport.hcl = Some(String::from(entry.1)),
                "ecl" => passport.ecl = Some(String::from(entry.1)),
                "pid" => passport.pid = Some(String::from(entry.1)),
                "cid" => passport.cid = Some(String::from(entry.1)),
                other => return Err(format!("Unknown key: {}", other)),
            };
        }

        Ok(passport)
    }
}

impl Passport {
    fn check_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn check_part2(&self) -> bool {
        if !self.check_part1() {
            return false;
        }

        let byr: u32 = self.byr.clone().unwrap().parse().unwrap();

        if !(1920..=2020).contains(&byr) {
            return false;
        }

        let iyr: u32 = self.iyr.clone().unwrap().parse().unwrap();

        if !(2010..=2020).contains(&iyr) {
            return false;
        }

        let eyr: u32 = self.eyr.clone().unwrap().parse().unwrap();

        if !(2020..=2030).contains(&eyr) {
            return false;
        }

        let hgt = self.hgt.clone().unwrap();
        let unit: &str = &hgt[hgt.len() - 2..];
        let val: u32 = hgt[..hgt.len() - 2].parse().unwrap();

        if unit != "in" && unit != "cm" {
            return false;
        }

        if unit == "in" && !(59..=76).contains(&val) || unit == "cm" && !(150..=193).contains(&val) {
            return false;
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d|[a-z]){6}$").unwrap();
        }

        if !RE.is_match(&self.hcl.clone().unwrap()) {
            return false;
        }

        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if !colors.contains(&&self.ecl.clone().unwrap()[..]) {
            return false;
        }

        lazy_static! {
            static ref RE2: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if !RE2.is_match(&self.pid.clone().unwrap()) {
            return false;
        }

        true
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(|s| s.try_into().unwrap()).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|&x| x.clone().check_part1()).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|&x| x.clone().check_part2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn sample1_part1() {
        let sample = generator(&fs::read_to_string("tests/day04/sample1").unwrap());
        assert_eq!(part1(&sample), 2);
    }

    #[test]
    fn sample1_part2() {
        let sample = generator(&fs::read_to_string("tests/day04/sample2").unwrap());

        assert_eq!(part2(&sample), 4);
    }
}
