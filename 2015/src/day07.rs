use itertools::Itertools as _;
use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<(String, String)> {
    input.lines()
        .map(|x| x.split(" -> ").collect::<Vec<&str>>())
        .map(|l| (l[0].to_owned(), l[1].to_owned()))
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(String, String)]) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut input = input.to_vec();

    while !input.is_empty() {
        let mut new_input: Vec<(String, String)> = vec![];
        let iter = input.iter();

        for next in iter {
            let key = next.1.clone();
            let value = next.0.split_whitespace().collect_vec();
            match value.len() {
                1 => {
                    if let Ok(v) = value[0].parse() {
                        map.insert(key, v);
                    } else {
                        let a = map.get(value[0]).cloned();
                        if let Some(a) = a {
                            map.insert(key, a);
                        } else {
                            new_input.push(next.clone());
                        }
                    }
                },
                2 => {
                    let a = map.get(value[1]).cloned();
                    if let Some(a) = a {
                        map.insert(key, !a);
                    } else {
                        new_input.push(next.clone());
                    }
                },
                3 => {
                    let a = value[0].parse::<usize>();
                    let b = map.get(value[2]).cloned();

                    if let Some(b) = b {
                        if let Ok(a) = a {
                            map.insert(key, calc_val(a, b, value[1]));
                        } else if let Some(a) = map.get(value[0]).cloned() {
                            map.insert(key, calc_val(a, b, value[1]));
                        } else {
                            new_input.push(next.clone());
                        }
                    } else {
                        new_input.push(next.clone());
                    }
                },
                _ => unreachable!()
            }
        }

        input = new_input;
    }

    *map.get("a").unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(String, String)]) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut input = input.to_vec();

    while !input.is_empty() {
        let mut new_input: Vec<(String, String)> = vec![];
        let iter = input.iter();

        for next in iter {
            let key = next.1.clone();
            let value = if &key == "b" {
                vec!["46065"]
            } else {
                next.0.split_whitespace().collect_vec()
            };
            
            match value.len() {
                1 => {
                    if let Ok(v) = value[0].parse() {
                        map.insert(key, v);
                    } else {
                        let a = map.get(value[0]).cloned();
                        if let Some(a) = a {
                            map.insert(key, a);
                        } else {
                            new_input.push(next.clone());
                        }
                    }
                },
                2 => {
                    let a = map.get(value[1]).cloned();
                    if let Some(a) = a {
                        map.insert(key, !a);
                    } else {
                        new_input.push(next.clone());
                    }
                },
                3 => {
                    let a = value[0].parse::<usize>();
                    let b = map.get(value[2]).cloned();

                    if let Some(b) = b {
                        if let Ok(a) = a {
                            map.insert(key, calc_val(a, b, value[1]));
                        } else if let Some(a) = map.get(value[0]).cloned() {
                            map.insert(key, calc_val(a, b, value[1]));
                        } else {
                            new_input.push(next.clone());
                        }
                    } else {
                        new_input.push(next.clone());
                    }
                },
                _ => unreachable!()
            }
        }

        input = new_input;
    }

    *map.get("a").unwrap()
}


fn calc_val(a: usize, b: usize, ops: &str)  -> usize {
    match ops {
        "AND" => a & b,
        "OR" => a | b,
        "LSHIFT" => a << b,
        "RSHIFT" => a >> b,
        _ => unreachable!()
    }
}