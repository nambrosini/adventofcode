use itertools::Itertools;

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[aoc(day22, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let deck = (0..10007).collect_vec();

    let deck = execute1(input, deck);

    deck.iter().position(|&x| x == 2019).unwrap()
}

fn execute1(input: &[Instruction], deck: Vec<i32>) -> Vec<i32> {
    let mut deck = deck;
    
    for i in input {
        deck = i.execute(&deck);
    }

    deck
}

pub enum Instruction {
    Deal,
    Cut(i32),
    Inc(i32)
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let s = s.split(' ').collect_vec();

        if s[0] == "deal" {
            if s[1] == "with" {
                let v = s[3].parse::<i32>().unwrap();
                Self::Inc(v)
            } else {
                Self::Deal
            }
        } else {
            let v = s[1].parse::<i32>().unwrap();
            Self::Cut(v)
        }
    }
}

impl Instruction {
    fn execute(&self, cards: &[i32]) -> Vec<i32> {
        match self {
            Self::Deal => deal(cards),
            Self::Cut(v) => cut(cards, *v),
            Self::Inc(v) => inc(cards, *v)
        }
    }
}

fn deal(cards: &[i32]) -> Vec<i32> {
    cards.iter().rev().copied().collect_vec()
}

fn cut(cards: &[i32], v: i32) -> Vec<i32> {
    let mut new_deck = Vec::new();

    if v >= 0 {
        new_deck.extend_from_slice(&cards[v as usize..]);
        new_deck.extend_from_slice(&cards[..v as usize]);
    } else {
        let v: usize = cards.len() - v.abs() as usize;
        new_deck.extend_from_slice(&cards[v..]);
        new_deck.extend_from_slice(&cards[..v]);
    }

    new_deck
}

fn inc(cards: &[i32], v: i32) -> Vec<i32> {
    let mut new_deck = vec![0; cards.len()];
    let mut new_index = 0;

    for i in 0..cards.len() {
        new_deck[new_index] = cards[i];
        new_index = (new_index + v as usize) % cards.len();
    }

    new_deck
}

#[test]
fn test_deal() {
    let s = "deal into new stack";

    assert_eq!(execute1(&generator(s), (0..10).collect_vec()), vec![9,8,7,6,5,4,3,2,1,0]);
}

#[test]
fn test_cut() {
    let s = "cut 3";

    assert_eq!(execute1(&generator(s), (0..10).collect_vec()), vec![3,4,5,6,7,8,9,0,1,2]);
}

#[test]
fn test_dealn() {
    let s = "deal with increment 3";

    assert_eq!(execute1(&generator(s), (0..10).collect_vec()), vec![0,7,4,1,8,5,2,9,6,3]);
}

#[test]
fn test_cunt() {
    let s = "cut -4";

    assert_eq!(execute1(&generator(s), (0..10).collect_vec()), vec![6,7,8,9,0,1,2,3,4,5]);
}