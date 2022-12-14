use std::collections::HashMap;

#[aoc_generator(day14)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn get_masked_value(mask: &str, value: i64) -> i64 {
    let mut bits: Vec<i64> = vec![0; mask.len()];
    let mut v = value;
    let mut counter = bits.len() - 1;

    while v > 0 {
        let bit = v & 1;
        bits[counter] = bit;
        v >>= 1;
        counter -= 1;
    }

    let mask: Vec<char> = mask.chars().collect();

    for i in 0..mask.len() {
        if mask[i] == '1' {
            bits[i] = 1;
        } else if mask[i] == '0' {
            bits[i] = 0;
        }
    }

    let mut res: i64 = 0;
    let min = bits.iter().enumerate().find(|(_, &x)| x == 1).unwrap().0;

    for b in bits.iter().skip(min) {
        res <<= 1;
        res += b;
    }
    res
}

fn get_addresses(mask: &str, value: i64) -> Vec<i64> {
    let mut bits: Vec<char> = vec!['0'; mask.len()];
    let mut v = value;
    let mut counter = bits.len() - 1;

    while v > 0 {
        let bit = v & 1;
        bits[counter] = if bit == 0 { '0' } else { '1' };
        v >>= 1;
        counter -= 1;
    }

    let mask: Vec<char> = mask.chars().collect();

    for i in 0..mask.len() {
        if mask[i] != '0' {
            bits[i] = mask[i];
        }
    }

    let bits_x_pos: Vec<usize> = bits
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == 'X')
        .map(|(i, _)| i)
        .collect();

    let mut bits_results: Vec<Vec<char>> = vec![];

    for i in 0..=(2usize.pow(bits_x_pos.len() as u32)) {
        let mut b = bits.clone();

        for j in 0..bits_x_pos.len() {
            let v = (i >> j) & 1;
            let v = if v == 0 { '0' } else { '1' };

            b[bits_x_pos[j]] = v;
        }

        bits_results.push(b.clone());
    }

    let mut results: Vec<i64> = vec![];

    for b in bits_results {
        let mut res = 0;
        let min = b.iter().enumerate().find(|(_, &x)| x == '1').unwrap().0;
        for &i in b.iter().skip(min) {
            res <<= 1;
            res += if i == '1' { 1 } else { 0 };
        }

        results.push(res);
    }

    results
}

#[aoc(day14, part1)]
fn part1(input: &[String]) -> i64 {
    let mut mem: HashMap<usize, i64> = HashMap::new();
    let mut mask = "";

    for l in input {
        let split: Vec<&str> = l.split(" = ").collect();

        if split[0] == "mask" {
            mask = split[1];
        } else {
            let index = split[0];
            let index: usize = index[4..index.len() - 1].parse().unwrap();
            let value: i64 = split[1].parse().unwrap();
            let m = mem.entry(index).or_insert(0);
            *m = get_masked_value(mask, value);
        }
    }

    mem.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &[String]) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask = "";

    for l in input {
        let split: Vec<&str> = l.split(" = ").collect();

        if split[0] == "mask" {
            mask = split[1];
        } else {
            let address: i64 = split[0][4..split[0].len() - 1].parse().unwrap();
            let value: u64 = split[1].parse().unwrap();
            let address = get_addresses(mask, address);
            for x in address {
                mem.insert(x as usize, value);
            }
        }
    }

    mem.values().sum()
}

#[test]
fn test1() {
    let s = generator(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0");

    assert_eq!(part1(&s), 165);
}

#[test]
fn test2() {
    let s = generator("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1");

    assert_eq!(part2(&s), 208);
}
