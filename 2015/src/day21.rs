use itertools::Itertools;
use std::ops::Add;

#[derive(Clone)]
pub struct Character {
    hitpoints: usize,
    damage: usize,
    armor: usize,
}

impl Character {
    fn rounds(&self, opponent: &Character) -> u16 {
        let round_damage: f64 = if opponent.armor >= self.damage {
            1
        } else {
            self.damage - opponent.armor
        } as f64;

        ((opponent.hitpoints as f64) / round_damage).ceil() as u16
    }

    fn beats(&self, opponent: &Character) -> bool {
        let a = self.rounds(opponent);
        let b = opponent.rounds(&self);

        a <= b
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl<'a, 'b> Add<&'b Inventory> for &'a Inventory {
    type Output = Inventory;

    fn add(self, rhs: &'b Inventory) -> Self::Output {
        Inventory {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Character {
    let val = input
        .lines()
        .map(|x| x.split(' ').last().unwrap())
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let boss = Character {
        hitpoints: val[0],
        damage: val[1],
        armor: val[2],
    };

    boss
}

#[aoc(day21, part1)]
pub fn part1(boss: &Character) -> usize {
    let weapons = [
        Inventory {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Inventory {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Inventory {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Inventory {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Inventory {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = [
        Inventory {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Inventory {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Inventory {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Inventory {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Inventory {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = [
        Inventory {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Inventory {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Inventory {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Inventory {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Inventory {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Inventory {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut minimal_inventory = Inventory {
        cost: usize::MAX,
        damage: 0,
        armor: 0,
    };

    for weapon in &weapons {
        let mut inventories = vec![];
        inventories.push(weapon.clone());

        for armor in &armor {
            inventories.push(weapon + armor);
        }

        let mut additional = vec![];
        for rings in rings.iter().combinations(2) {
            for inventory in inventories.iter() {
                additional.push(inventory + rings[0]);
                additional.push(inventory + rings[1]);
                additional.push(inventory + &(rings[0] + rings[1]));
            }
        }

        inventories.extend(additional);

        for inventory in inventories {
            let player = Character {
                hitpoints: 100,
                damage: inventory.damage,
                armor: inventory.armor,
            };

            let winner = player.beats(&boss);

            if winner && inventory.cost < minimal_inventory.cost {
                minimal_inventory = inventory.clone();
            }
        }
    }
    minimal_inventory.cost
}

#[aoc(day21, part2)]
pub fn part2(boss: &Character) -> usize {
    let weapons = [
        Inventory {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Inventory {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Inventory {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Inventory {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Inventory {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = [
        Inventory {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Inventory {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Inventory {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Inventory {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Inventory {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = [
        Inventory {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Inventory {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Inventory {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Inventory {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Inventory {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Inventory {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut maximal_inventory = Inventory {
        cost: 0,
        damage: 0,
        armor: 0,
    };

    for weapon in &weapons {
        let mut inventories = Vec::new();
        inventories.push(weapon.clone());

        for armor in &armor {
            inventories.push(weapon + armor);
        }

        let mut additional = Vec::with_capacity(rings.len() * 3);
        for rings in rings.iter().combinations(2) {
            for inventory in inventories.iter() {
                additional.push(inventory + &rings[0]);
                additional.push(inventory + &rings[1]);
                additional.push(inventory + &(rings[0] + rings[1]));
            }
        }

        inventories.extend(additional);
        for inventory in &inventories {
            let player = Character {
                hitpoints: 100,
                damage: inventory.damage,
                armor: inventory.armor,
            };

            let winner = player.beats(&boss);
            if !winner && inventory.cost >= maximal_inventory.cost {
                maximal_inventory = inventory.clone();
            }
        }
    }
    maximal_inventory.cost
}
