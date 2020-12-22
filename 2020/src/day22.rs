use itertools::Itertools as _;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(PartialEq)]
enum Player {
    Player1,
    Player2,
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let split = input.split("\n\n").collect_vec();

    let player1 = split[0]
        .lines()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();

    let player2 = split[1]
        .lines()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();

    (player1, player2)
}

fn calc_score(x: &[usize]) -> usize {
    x.iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, &e)| sum + e * (i + 1))
}

#[aoc(day22, part1)]
pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut p1 = input.0.clone();
    let mut p2 = input.1.clone();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.remove(0);
        let p2_card = p2.remove(0);

        if p1_card > p2_card {
            p1.extend_from_slice(&[p1_card, p2_card]);
        } else {
            p2.extend_from_slice(&[p2_card, p1_card]);
        }
    }

    if p2.is_empty() {
        calc_score(&p1)
    } else {
        calc_score(&p2)
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut p1 = input.0.clone();
    let mut p2 = input.1.clone();

    match recursive_cards(&mut p1, &mut p2) {
        Player::Player1 => calc_score(&p1),
        Player::Player2 => calc_score(&p2),
    }
}

fn recursive_cards(p1: &mut Vec<usize>, p2: &mut Vec<usize>) -> Player {
    let mut previous: Vec<u64> = vec![];

    while !p1.is_empty() && !p2.is_empty() {
        let mut hash = DefaultHasher::new();
        p1.hash(&mut hash);
        p2.hash(&mut hash);
        let hash = hash.finish();

        if previous.contains(&hash) {
            return Player::Player1;
        } else {
            previous.push(hash);
        }

        let p1_card = p1.remove(0);
        let p2_card = p2.remove(0);

        if p1_card <= p1.len() && p2_card <= p2.len() {
            if recursive_cards(
                &mut p1[..p1_card].to_vec().clone(),
                &mut p2[..p2_card].to_vec().clone(),
            ) == Player::Player1
            {
                p1.extend_from_slice(&[p1_card, p2_card]);
            } else {
                p2.extend_from_slice(&[p2_card, p1_card]);
            }
        } else if p1_card > p2_card {
            p1.extend_from_slice(&[p1_card, p2_card]);
        } else {
            p2.extend_from_slice(&[p2_card, p1_card]);
        }
    }

    if p2.is_empty() {
        Player::Player1
    } else {
        Player::Player2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = generator(&std::fs::read_to_string("tests/day22/sample1.txt").unwrap());

        assert_eq!(part1(&s), 306);
    }

    #[test]
    fn sample1_test2() {
        let s = generator(&std::fs::read_to_string("tests/day22/sample1.txt").unwrap());

        assert_eq!(part2(&s), 291);
    }
}
