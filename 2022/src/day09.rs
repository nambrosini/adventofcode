use std::collections::HashSet;

type Movement = (String, usize);

#[aoc_generator(day09)]
pub fn generator(input: &str) -> Vec<Movement> {
    input.lines().map(|l| {
        let split: Vec<&str> = l.split_whitespace().collect();
        (split[0].to_string(), split[1].parse().unwrap())
    }).collect()
}

#[aoc(day09, part1)]
pub fn part1(input: &[Movement]) -> usize {
    tails(input, 2).len()
}

#[aoc(day09, part2)]
pub fn part2(input: &[Movement]) -> usize {
    tails(input, 10).len()
}

fn tails(lines: &[Movement], length: usize) -> HashSet<Knot> {
    let mut rope = vec![Knot::default(); length];
    let mut res = HashSet::new();
    res.insert(*rope.last().unwrap());

    for m in lines {
        for _ in 0..m.1 {
            move_head(&mut rope, &m.0);
            res.insert(*rope.last().unwrap());
        }
    }

    res
}

fn move_head(rope: &mut Vec<Knot>, dir: &str) {
    match dir {
        "U" => rope[0].irow -= 1,
        "D" => rope[0].irow += 1,
        "L" => rope[0].icol -= 1,
        "R" => rope[0].icol += 1,
        _ => unreachable!()
    }

    for i in 1..rope.len() {
        let drow = rope[i - 1].irow - rope[i].irow;
        let dcol = rope[i - 1].icol - rope[i].icol;

        if drow.abs() > 1 || dcol.abs() > 1 {
            rope[i] = Knot {
                irow: rope[i].irow + drow.signum(),
                icol: rope[i].icol + dcol.signum()
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Knot {
    irow: i32,
    icol: i32,
}

#[test]
fn test() {
    let s = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let got = part1(&generator(s));

    assert_eq!(got, 13);
}

#[test]
fn test2() {
    let s = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let got = part2(&generator(s));

    assert_eq!(got, 36);
}