use std::collections::HashMap;

type Input = (
    HashMap<String, Vec<(usize, usize)>>,
    Vec<usize>,
    Vec<Vec<usize>>,
);

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Input {
    let split: Vec<&str> = input.split("\n\n").collect();

    let mut valid_fields: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    for i in split[0].lines() {
        let second_split: Vec<&str> = i.split(": ").collect();

        let key = second_split[0].to_owned();

        let value: Vec<(usize, usize)> = second_split[1]
            .split(" or ")
            .map(|x| x.split('-').collect::<Vec<&str>>())
            .map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
            .collect();

        valid_fields.insert(key, value);
    }

    let my_ticket: Vec<usize> = split[1]
        .lines()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let other_tickets: Vec<Vec<usize>> = split[2]
        .lines()
        .enumerate()
        .filter(|(i, _)| i > &0)
        .map(|(_, x)| {
            x.split(',')
                .map(|y| y.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    (valid_fields, my_ticket, other_tickets)
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    let ranges: Vec<usize> = input
        .0
        .values()
        .flat_map(|x| x.iter().flat_map(|y| (y.0..=y.1).collect::<Vec<usize>>()))
        .collect();

    input
        .2
        .iter()
        .flatten()
        .filter(|&x| !ranges.contains(x))
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    let ranges: Vec<usize> = input
        .0
        .values()
        .flat_map(|x| x.iter().flat_map(|y| (y.0..=y.1).collect::<Vec<usize>>()))
        .collect();

    let tickets: Vec<Vec<usize>> = input
        .2
        .iter()
        .filter(|x| {
            for i in x.iter() {
                if !ranges.contains(&i) {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    let mut pos: Vec<Vec<usize>> = vec![];

    for i in 0..tickets[0].len() {
        let mut v = vec![];
        for j in 0..tickets.len() {
            v.push(tickets[j][i]);
        }
        pos.push(v);
    }

    let mut fields: Vec<Vec<String>> = vec![];

    for i in pos {
        let key: Vec<String> = input
            .0
            .iter()
            .filter_map(|(key, val)| {
                let mut b = true;
                for j in i.clone() {
                    if !(val[0].0 <= j && val[0].1 >= j || val[1].0 <= j && val[1].1 >= j) {
                        b = false;
                    }
                }

                if b {
                    Some(key.to_owned())
                } else {
                    None
                }
            })
            .collect();

        fields.push(key);
    }

    let mut keys = HashMap::<usize, String>::new();

    let mut index = 0;

    while keys.len() != fields.len() {
        println!("{:?}", fields);
        println!("{:?}", keys);
        match fields[index].len() {
            1 => {
                keys.insert(index, fields[index][0].clone());
                fields[index] = vec![];
            }
            d if d > 1 => {
                let mut v = vec![];
                let values: Vec<String> = keys.values().cloned().collect();

                for f in fields[index].clone() {
                    if !values.contains(&f) {
                        v.push(f);
                    }
                }

                fields[index] = v;
            }
            _ => {}
        }
        index = (index + 1) % fields.len();
    }

    let mut res = 1;

    for k in keys {
        if k.1.contains("departure") {
            res *= input.1[k.0];
        }
    }

    res
    // fields.iter()
    //     .enumerate()
    //     .filter(|(_, x)| x.contains(&"departure".to_owned()))
    //     .map(|(i, _)| input.1[i])
    //     .fold(1, |res, x| res * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator(&std::fs::read_to_string("tests/day16/sample1").unwrap());

        assert_eq!(part1(&s), 71);
    }

    #[test]
    fn sample2_test2() {
        let s = generator(&std::fs::read_to_string("tests/day16/sample2").unwrap());

        assert_eq!(part2(&s), 12 * 11);
    }
}
