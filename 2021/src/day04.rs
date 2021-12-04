use itertools::Itertools;

#[aoc_generator(day04)]
pub fn generator(input: &str) -> (Vec<usize>, Vec<Board>) {
    let lines = input.lines().collect_vec();

    let drawn = lines[0]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect_vec();

    let boards = input.split("\n\n").skip(1).collect_vec();

    let boards = boards
        .iter()
        .map(|board| {
            let board = board
                .lines()
                .map(|line| {
                    line.split(' ')
                        .filter(|&x| !x.is_empty())
                        .map(|line| line.trim().parse().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            Board {
                board
            }
        })
        .collect_vec();

    (drawn, boards)
}

#[aoc(day04, part1)]
pub fn part1((extracted, boards): &(Vec<usize>, Vec<Board>)) -> usize {
    for i in 5..extracted.len() {
        for b in boards {
            if b.check_board(&extracted[..=i]) {
                return b.calc_score(&extracted[..=i]);
            }
        }
    }

    unreachable!()
}

#[aoc(day04, part2)]
pub fn part2((extracted, boards): &(Vec<usize>, Vec<Board>)) -> usize {
    let mut boards = boards.clone();
    let mut last_winning_board: Board = boards[0].clone();
    let mut last_winning_index = 5;

    for i in 5..extracted.len() {
        let mut new_boards = vec![];
        for b in &boards {
            if !b.check_board(&extracted[..=i]) {
                new_boards.push(b.clone());
            } else {
                last_winning_board = b.clone();
                last_winning_index = i;
            }
        }
        boards = new_boards;
    }

    last_winning_board.calc_score(&extracted[..=last_winning_index])
}

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<usize>>
}

impl Board {
    fn check_board(&self, extracted: &[usize]) -> bool {
        for (i, r) in self.board.iter().enumerate() {
            let mut row = true;
            let mut col = true;
            for (j, e) in r.iter().enumerate() {
                if !extracted.contains(e) {
                    row = false;
                }

                if !extracted.contains(&self.board[j][i]) {
                    col = false;
                }

                if !row && !col {
                    break;
                }
            }

            if row || col {
                return true;
            }
        }

        false
    }

    fn calc_score(&self, extracted: &[usize]) -> usize {
        self.board.iter()
            .flatten()
            .filter(|x| !extracted.contains(x))
            .sum::<usize>() * extracted.last().unwrap()
    }
}