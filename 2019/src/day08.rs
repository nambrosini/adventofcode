use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Layer = Vec<usize>;

#[aoc_generator(day08, part1)]
pub fn generator(input: &str) -> Vec<Layer> {
    let input = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    input.chunks(WIDTH * HEIGHT).map(|x| x.to_vec()).collect()
}

#[aoc_generator(day08, part2)]
pub fn generator2(input: &str) -> Vec<usize> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}

#[aoc(day08, part1)]
pub fn part1(input: &[Layer]) -> usize {
    let min = input.iter()
        .min_by_key(|x| x.iter().filter(|&&x| x == 0).count())
        .unwrap();

    let ones = min.iter().filter(|&&x| x == 1).count();
    let twos = min.iter().filter(|&&x| x == 2).count();

    ones * twos
}

#[aoc(day08, part2)]
pub fn part2(input: &[usize]) -> String {
    let mut pic = vec![];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let val = input
                .chunks(WIDTH * HEIGHT)
                .map(|layer| layer[(i * WIDTH) + j])
                .find(|&x| x != 2)
                .unwrap_or(2);

            pic.push(val)
        }
    }

    let mut s = String::from("\n");

    for line in pic.chunks(WIDTH) {
        for i in line {
            match i {
                1 => s.push('▓'),
                _ => s.push(' '),
            }
        }
        s.push('\n');
    }

    s
}

#[test]
fn test1() {
    let s = vec![
        vec![1, 2, 0, 2, 2, 1],
        vec![7, 8, 0, 0, 1, 2]
    ];

    assert_eq!(part1(&s), 6);
}