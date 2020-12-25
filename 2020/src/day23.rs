use itertools::Itertools as _;

#[aoc_generator(day23, part1)]
pub fn generator_part1(input: &str) -> Game {
    Game::new(input)
}

#[aoc_generator(day23, part2)]
pub fn generator_part2(input: &str) -> Game {
    Game::new_with_extra(input)
}

fn to_linked_vec(cups: &[usize]) -> Vec<usize> {
    let mut cups_linked = vec![0; cups.len() + 1];
    for window in cups.windows(2) {
        cups_linked[window[0]] = window[1];
    }
    cups_linked[cups[cups.len() - 1]] = cups[0];
    cups_linked
}

#[derive(Clone)]
pub struct Game {
    cups: Vec<usize>,
    current_cup: usize,
}

impl Game {
    fn new(input: &str) -> Self {
        let cups = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();

        Self {
            cups: to_linked_vec(&cups),
            current_cup: cups[0],
        }
    }

    fn new_with_extra(input: &str) -> Self {
        let mut cups = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();
        cups.extend(cups.iter().copied().max().unwrap() + 1..=1000000);

        Self {
            cups: to_linked_vec(&cups),
            current_cup: cups[0],
        }
    }

    fn play_game(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.play_round();
        }
    }

    fn play_round(&mut self) {
        let mut dest_cup = if self.current_cup == 1 {
            self.cups.len() - 1
        } else {
            self.current_cup - 1
        };

        let picked_1 = self.cups[self.current_cup];
        let picked_2 = self.cups[picked_1];
        let picked_3 = self.cups[picked_2];
        while [picked_1, picked_2, picked_3].contains(&dest_cup) {
            dest_cup = if dest_cup == 1 {
                self.cups.len() - 1
            } else {
                dest_cup - 1
            };
        }

        self.cups[self.current_cup] = self.cups[picked_3];
        let post_dest = self.cups[dest_cup];
        self.cups[dest_cup] = picked_1;
        self.cups[picked_1] = picked_2;
        self.cups[picked_2] = picked_3;
        self.cups[picked_3] = post_dest;
        self.current_cup = self.cups[self.current_cup];
    }

    fn get_representation(&self) -> String {
        let mut s = String::new();
        let mut i = self.cups[1];
        while i != 1 {
            s.push_str(&i.to_string());
            i = self.cups[i];
        }

        s
    }

    fn get_score(&self) -> usize {
        let el1 = self.cups[1];
        let el2 = self.cups[el1];
        el1 * el2
    }
}

#[aoc(day23, part1)]
pub fn part1(game: &Game) -> String {
    let mut game = game.clone();
    game.play_game(100);
    game.get_representation()
}

#[aoc(day23, part2)]
pub fn part2(game: &Game) -> usize {
    let mut game: Game = game.clone();
    game.play_game(10_000_000);
    game.get_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut s = generator_part1("389125467");

        assert_eq!(part1(&mut s), "67384529");
    }

    #[test]
    fn sample1_test2() {
        let s = generator_part2("389125467");

        assert_eq!(part2(&s), 149245887792);
    }
}
