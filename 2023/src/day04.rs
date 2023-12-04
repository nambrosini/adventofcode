use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day04)]
pub fn generate(input: &str) -> Vec<usize> {
    let cards: Vec<usize> = input
        .lines()
        .map(|l| {
            let card: Vec<&str> = l.split(": ").collect();
            let card = card[1];
            let numbers: String = card.replace(" |", "");
            let numbers: Vec<u32> = numbers
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let count = numbers.len();
            count - numbers.iter().unique().count()
        })
        .collect();

    cards
}

#[aoc(day04, part1)]
pub fn part1(cards: &[usize]) -> i32 {
    cards
        .iter()
        .map(|x| if *x == 0 { 0 } else { 2i32.pow((x - 1) as u32) })
        .sum()
}

#[aoc(day04, part2)]
pub fn part2(cards: &[usize]) -> u32 {
    let mut map: HashMap<usize, u32> = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        let entry = map.entry(i).or_insert(0);
        *entry += 1;
        let c = *entry;
        for k in i + 1..i + 1 + card {
            let entry = map.entry(k).or_insert(0);
            *entry += c;
        }
    }
    map.values().sum()
}

#[test]
fn test_part1() {
    let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(part1(&generate(s)), 13)
}

#[test]
fn test_part2() {
    let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(part2(&generate(s)), 30)
}
