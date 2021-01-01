use std::cmp::max;
use std::collections::HashMap;
use itertools::Itertools;

// #[derive(Debug, Clone)]
// pub struct Aunt {
//     id: Option<usize>,
//     children: Option<usize>,
//     cats: usize,
//     samoyeds: usize,
//     pomerians: usize,
//     akitas: usize,
//     vizslas: usize,
//     goldfish: usize,
//     trees: usize,
//     cars: usize,
//     perfumes: usize
// }
//
// impl Aunt {
//     fn new() -> Self {
//         Self {
//
//         }
//     }
// }

// impl TryFrom<&str> for Aunt {
//     type Error = String;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let split = value.split(' ').collect_vec();
//
//     }
// }

type MapType = HashMap<usize, HashMap<String, usize>>;

#[aoc_generator(day16)]
pub fn generator(input: &str) -> MapType {
    let mut map: MapType = MapType::new();

    for l in input.lines() {
        let l = l.replace(":", "").replace(",", "");
        let split = l.split(' ').collect_vec();
        let id = split[1].parse().unwrap();

        let mut aunt = HashMap::new();

        for i in (2..split.len()).step_by(2) {
            aunt.insert(split[i].to_owned(), split[i + 1].parse().unwrap());
        }

        map.insert(id, aunt);
    }

    map
}

#[aoc(day16, part1)]
pub fn part1(aunts: &MapType) -> usize {
    let mut aunt = HashMap::new();
    aunt.insert("children".to_owned(), 3);
    aunt.insert("cats".to_owned(), 7);
    aunt.insert("samoyeds".to_owned(), 2);
    aunt.insert("pomeranians".to_owned(), 3);
    aunt.insert("akitas".to_owned(), 0);
    aunt.insert("vizslas".to_owned(), 0);
    aunt.insert("goldfish".to_owned(), 5);
    aunt.insert("trees".to_owned(), 3);
    aunt.insert("cars".to_owned(), 2);
    aunt.insert("perfumes".to_owned(), 2);

    let mut max = 0;
    let mut aunt_id = 0;

    for (id, fields) in aunts {
        let mut count = 0;

        for (k, v) in fields {
            if aunt[k] == *v {
                count += 1;
            }
        }

        if count > max {
            aunt_id = *id;
            max = count;
        }
    }

    aunt_id
}

#[aoc(day16, part2)]
pub fn part2(aunts: &MapType) -> usize {
    let mut aunt = HashMap::new();
    aunt.insert("children".to_owned(), 3);
    aunt.insert("cats".to_owned(), 7);
    aunt.insert("samoyeds".to_owned(), 2);
    aunt.insert("pomeranians".to_owned(), 3);
    aunt.insert("akitas".to_owned(), 0);
    aunt.insert("vizslas".to_owned(), 0);
    aunt.insert("goldfish".to_owned(), 5);
    aunt.insert("trees".to_owned(), 3);
    aunt.insert("cars".to_owned(), 2);
    aunt.insert("perfumes".to_owned(), 2);

    let mut max = 0;
    let mut aunt_id = 0;

    for (id, fields) in aunts {
        let mut count = 0;

        for (k, v) in fields {
            let check = match k.as_str() {
                "cats" | "trees" => aunt[k] < *v,
                "pomeranians" | "goldfish" => aunt[k] > *v,
                _ => aunt[k] == *v
            };

            if check {
                count += 1;
            }
        }

        if count > max {
            aunt_id = *id;
            max = count;
        }
    }

    aunt_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = std::fs::read_to_string("tests/day16/sample1.txt").unwrap();
        let generated = generator(&s);
        assert_eq!(part1(&generated), 62842880);
    }
}
