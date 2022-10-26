const SIZE: usize = 25 * 6;

#[aoc_generator(day08)]
pub fn generator(input: &str) -> Vec<Vec<usize>> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .chunks(SIZE)
        .map(|x| x.to_vec())
        .collect()
}

#[aoc(day08, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let v = input.iter()
        .min_by_key(|v| v.iter().filter(|x| x == &&0).count())
        .unwrap();

    let ones: usize = v.iter().filter(|&&x| x == 1).count();
    let twos: usize = v.iter().filter(|&&x| x == 2).count();

    ones * twos
}

#[aoc(day08, part2)]
pub fn part2(input: &[Vec<usize>]) -> usize {
    let mut result = [2; SIZE];

    for i in input {
        for (j, e) in result.iter_mut().enumerate().take(SIZE) {
            if *e == 2 {
                *e = i[j];
            }
        }
    }

    for x in result.chunks(25) {
        for y in x {
            if y == &2 || y == &0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }

    0
}