#[aoc_generator(day05)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|l| calc_id(&l))
        .collect()
}

pub fn calc_id(id: &[char]) -> usize {
    let (mut start_row, mut end_row) = (0, 127);
    for i in id.iter().take(7) {
        let c = i;

        match c {
            'F' => end_row -= (end_row - start_row + 1) / 2,
            'B' => start_row += (end_row - start_row + 1) / 2,
            _ => unreachable!(),
        }
    }

    let row = if id[6] == 'F' { end_row } else { start_row };

    let (mut start_col, mut end_col) = (0, 7);

    for i in id.iter().skip(7) {
        let c = i;

        match c {
            'L' => end_col -= (end_col - start_col + 1) / 2,
            'R' => start_col += (end_col - start_col + 1) / 2,
            _ => unreachable!(),
        }
    }

    let col = if id[id.len() - 1] == 'L' {
        end_col
    } else {
        start_col
    };

    row * 8 + col
}

#[aoc(day05, part1)]
pub fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[usize]) -> usize {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..=*max).find(|x| !input.contains(x)).unwrap()
}


#[test]
fn sample1_part1() {
    let s = generator("FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");

    assert_eq!(part1(&s), 820);
}

#[test]
fn sample1_part2() {
    let s = generator("FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");

    assert_eq!(part2(&s), 120);
}
