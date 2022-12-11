#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let mut count = 0;
    for x in 1..input.len() - 1 {
        for y in 1..input[x].len() - 1 {
            if is_seen(input, x, y) {
                count += 1;
            } else {
            }
        }
    }

    input.len() * 4 - 4 + count
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut best = 0;
    for (x, row) in input.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            let mut c = 0;
            for i in (0..y).rev() {
                if &row[i] < cell {
                    c += 1;
                } else {
                    c += 1;
                    break;
                }
            }
            let mut total = c;
            c = 0;

            for i in row.iter().skip(y + 1) {
                if i < cell {
                    c += 1;
                } else {
                    c += 1;
                    break;
                }
            }

            total *= c;
            c = 0;

            for i in (0..x).rev() {
                if &input[i][y] < cell {
                    c += 1;
                } else {
                    c += 1;
                    break;
                }
            }

            total *= c;
            c = 0;

            for i in x + 1..input.len() {
                if &input[i][y] < cell {
                    c += 1;
                } else {
                    c += 1;
                    break;
                }
            }

            total *= c;

            if total > best {
                best = total;
            }
        }
    }

    best
}

fn is_seen(map: &[Vec<usize>], x: usize, y: usize) -> bool {
    let row = &map[x];
    let col: Vec<usize> = map
        .iter()
        .flat_map(|v| {
            v.iter()
                .enumerate()
                .filter(|(j, _)| j == &y)
                .map(|(_, e)| *e)
                .collect::<Vec<usize>>()
        })
        .collect();
    let val = &map[x][y];

    row[..y].iter().all(|e| e < val)
        || row[y + 1..].iter().all(|e| e < val)
        || col[..x].iter().all(|e| e < val)
        || col[x + 1..].iter().all(|e| e < val)
}

#[test]
fn test() {
    let s = "30373
25512
65332
33549
35390";
    let got = part1(&generator(s));

    assert_eq!(21, got);
}

#[test]
fn test2() {
    let s = "30373
25512
65332
33549
35390";
    let got = part2(&generator(s));

    assert_eq!(8, got);
}
