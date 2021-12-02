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

#[test]
fn test1() {
    let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    assert_eq!(part1(&generator(s)), 2);
}

#[test]
fn test2_invalid() {
    let s = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    assert_eq!(part2(&generator(s)), 0);
}

#[test]
fn test2_valid() {
    let s = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(part2(&generator(s)), 4);
}