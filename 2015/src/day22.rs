use itertools::Itertools;

struct Character {
    hp: usize,
    damage: usize
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Character {
    let fields = input.lines()
        .map(|x| x.split(' ').collect_vec()[1])
        .map(|x| x.parse().unwrap())
        .collect_vec();

    Character {
        hp: fields[0],
        damage: fields[1]
    }
}

struct Spell {
    cost: usize,
    damage: Option<usize>,
    heal: Option<usize>,
    effect: Option<usize>,
    armor: Option<usize>,
    new_mana: Option<usize>
}

#[aoc(day22, part1)]
pub fn part1(boss: Character) -> usize {
    let spells = [
        Spell { cost: 53, damage: Some(4), heal: None, effect: None, armor: None, new_mana: None},
        Spell { cost: 73, damage: Some(2), heal: Some(2), effect: None, armor: None, new_mana: None},
        Spell { cost: 113, damage: None, heal: None, effect: Some(6), armor: Some(7), new_mana: None},
        Spell { cost: 173, damage: Some(4), heal: None, effect: None, armor: None, new_mana: None},
        Spell { cost: 53, damage: Some(4), heal: None, effect: None, armor: None, new_mana: None}
    ];


}