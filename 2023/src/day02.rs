use std::collections::HashMap;
use itertools::Itertools;

// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

#[aoc_generator(day02)]
pub fn generate(input: &str) -> Vec<Game> {
    let mut res = Vec::new();
    for l in input.lines() {
        let input: &str = l.split(": ").nth(1).unwrap();
        let split_hands: Vec<&str> = input.split("; ").collect();
        let mut game = Game::new();
        for hand in split_hands {
            let mut h: Hand = Hand::new();
            let cubes: Vec<&str> = hand.split(", ").collect();
            for cube in cubes {
                let mut split = cube.split(' ');
                let count: i32 = split.next().unwrap().parse().unwrap();
                let color = Color::new(split.next().unwrap());
                h.push((color, count));
            }
            game.push(h);
        }
        res.push(game);
    }
    res
}

#[aoc(day02, part1)]
pub fn part1(games: &[Game]) -> usize {
    let limits = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    games.iter()
        .enumerate()
        .filter(|(_, g)| g.iter().all(|h| check_hand(&limits, h)))
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day02, part2)]
pub fn part2(games: &[Game]) -> i32 {
    let games: Vec<Vec<(Color, i32)>> = games.iter()
        .map(|g| g.iter().flatten().copied().collect())
        .collect();

    let mut res = 0;
    for game in games {
        let mut r = 1;
        for color in [Color::Red, Color::Green, Color::Blue] {
            r *= game.iter()
                .filter(|(col, cou)| col == &color)
                .max_by_key(|(_, c)| c)
                .unwrap().1;
        }
        res += r;
    }
    res
}

fn check_hand(limit: &HashMap<Color, i32>, hand: &Hand) -> bool {
    for (color, count) in hand {
        if count > &limit[&color]  {
            return false;
        }
    }
    true
}

type Game = Vec<Hand>;
type Hand = Vec<(Color, i32)>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Color {
    Blue, Red, Green
}

impl Color {
    fn new(color: &str) -> Self {
        match color {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => unreachable!()
        }
    }
}

#[test]
fn test_1() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(part1(&generate(s)), 8);
}

#[test]
fn test_2() {
    let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(part2(&generate(s)), 2286);
}