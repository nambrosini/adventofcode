use itertools::Itertools;

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Cell> {
    input.chars().map(|c| c.into()).collect_vec()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Cell]) -> usize {
    solve(input, 40)
}

#[aoc(day18, part2)]
pub fn part2(input: &[Cell]) -> usize {
    solve(input, 400000)
}

fn solve(input: &[Cell], rows: usize) -> usize {
    let count_open = |row: &[Cell]| -> usize { row.iter().filter(|&&x| x == Cell::Safe).count() };
    let mut row: Vec<Cell> = input.to_vec();
    let mut count = count_open(&row);

    for _ in 1..rows {
        row = gen_next_line(&row);

        count += count_open(&row);
    }

    count
}

fn gen_next_line(previous: &[Cell]) -> Vec<Cell> {
    let mut new = vec![];

    for i in 0..previous.len() {
        if i == 0 {
            new.push(Cell::gen_from_previous(
                None,
                previous[i],
                previous.get(i + 1),
            ));
        } else {
            new.push(Cell::gen_from_previous(
                previous.get(i - 1),
                previous[i],
                previous.get(i + 1),
            ));
        }
    }

    new
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cell {
    Safe,
    Trap,
}

impl Cell {
    fn gen_from_previous(left: Option<&Cell>, center: Cell, right: Option<&Cell>) -> Self {
        let left = if let Some(left) = left {
            *left
        } else {
            Cell::Safe
        };
        let right = if let Some(right) = right {
            *right
        } else {
            Cell::Safe
        };

        if (center != Cell::Trap || center == Cell::Trap)
            && (right == Cell::Trap || right != Cell::Trap)
            && (right == Cell::Trap || left == Cell::Trap)
            && (left != Cell::Trap || right != Cell::Trap)
            && (left != Cell::Trap || left == Cell::Trap)
        {
            Cell::Trap
        } else {
            Cell::Safe
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Safe,
            '^' => Cell::Trap,
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_next_line() {
    let input = generator("..^^.");
    let expected = generator(".^^^^");

    assert_eq!(gen_next_line(&input), expected);
}

#[test]
fn test() {
    let input = generator(".^^.^.^^^^");

    assert_eq!(solve(&input, 10), 38);
}
