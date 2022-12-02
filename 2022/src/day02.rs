#[aoc_generator(day02, part1)]
pub fn generator_p1(input: &str) -> Vec<(Sign, Sign)> {
    input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            (split[0].into(), split[1].into())
        })
        .collect()
}

#[aoc_generator(day02, part2)]
pub fn generator_p2(input: &str) -> Vec<(Sign, Res)> {
    input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            (split[0].into(), split[1].into())
        })
        .collect()
}

#[aoc(day02, part1)]
pub fn part1(input: &[(Sign, Sign)]) -> usize {
    input.iter()
        .map(|(a, b)| {
            *b as usize + match a {
                Sign::Rock => match b {
                    Sign::Rock => Res::Draw as usize,
                    Sign::Paper => Res::Win as usize,
                    Sign::Scissor => Res::Loss as usize
                },
                Sign::Paper => match b {
                    Sign::Rock => Res::Loss as usize,
                    Sign::Paper => Res::Draw as usize,
                    Sign::Scissor => Res::Win as usize
                },
                Sign::Scissor => match b {
                    Sign::Rock => Res::Win as usize,
                    Sign::Paper => Res::Loss as usize,
                    Sign::Scissor => Res::Draw as usize
                }
            }
        })
        .sum()
}

#[aoc(day02, part2)]
pub fn part2(input: &[(Sign, Res)]) -> usize {
    input.iter()
        .map(|(a, b)| a.get_move(b) as usize + *b as usize)
        .sum()
}

#[derive(Copy, Clone)]
pub enum Res {
    Win = 6, Draw = 3, Loss = 0
}

impl From<&str> for Res {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Sign {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

impl Sign {
    fn get_move(&self, res: &Res) -> Self {
        match self {
            Sign::Rock => match res {
                Res::Win => Sign::Paper,
                Res::Draw => Sign::Rock,
                Res::Loss => Sign::Scissor
            }
            Sign::Paper => match res {
                Res::Win => Sign::Scissor,
                Res::Draw => Sign::Paper,
                Res::Loss => Sign::Rock
            }
            Sign::Scissor => match res {
                Res::Win => Sign::Rock,
                Res::Draw => Sign::Scissor,
                Res::Loss => Sign::Paper
            }
        }
    }
}

impl From<&str> for Sign {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!()
        }
    }
}

#[test]
fn test1() {
    let s = "A Y
B X
C Z";
    let want = 15;
    let got = part1(&generator(s));

    assert_eq!(want, got);
}