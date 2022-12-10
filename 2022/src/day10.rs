#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|x| x.into())
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];
    let v = get_vec(instructions);
    println!("{:?}", v);
    get_vec(instructions).iter()
        .filter(|(cycle, _)| cycles.contains(cycle))
        .map(|(cycle, x)| *cycle * x)
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(instructions: &[Instruction]) -> String {
    let mut screen = String::from("\n");
    for (cycle, x) in get_vec(instructions) {
        let sprite_middle = x;
        let screen_column = (cycle - 1) % 40;

        screen.push(if (sprite_middle - screen_column).abs() < 2 { '█' } else { '.' });

        if screen_column == 39 {
            screen.push('\n');
        }
    }
    screen
}

fn get_vec(instructions: &[Instruction]) -> Vec<(i32, i32)> {
    let mut res = vec![];
    let (mut cycle, mut x) = (0, 1);
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                res.push((cycle, x));
            },
            Instruction::Add(val) => {
                cycle += 1;
                res.push((cycle, x));
                cycle += 1;
                res.push((cycle, x));
                x += val;
            }
        }
    }

    res
}

pub enum Instruction {
    Noop,
    Add(i32)
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        match split[0] {
            "noop" => Self::Noop,
            "addx" => Self::Add(split[1].parse().unwrap()),
            _ => unreachable!()
        }
    }
}

#[test]
fn test() {
    let s = std::fs::read_to_string("tests/test10.txt").unwrap();
    let got = part1(&generator(&s));

    assert_eq!(got, 13140);
}