use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day10, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut line: Vec<usize> = (0..256).collect_vec();

    let mut start = 0;

    let len = &line.len();

    for (skip, value) in input.iter().enumerate() {
        for i in 0..value / 2 {
            line.swap((start + i) % len, (start + value - 1 - i) % len)
        }
        start += (skip + value) % len;
    }

    line[0] * line[1]
}

#[aoc(day10, part2)]
pub fn part2(input: &[usize]) -> String {
    let standard_lengths: Vec<usize> = vec![17, 31, 73, 47, 23];
    let mut lengths: Vec<usize> = input.iter().join(",").chars().map(|c| c as usize).collect();
    lengths.extend(standard_lengths);

    let mut start: usize = 0;
    let mut skip = 0;
    let iterations = 64;

    let mut line: Vec<i32> = (0..256).collect();
    let length = &line.len();
    //println!("{:?}", line);
    for _ in 0..iterations {
        for value in lengths.iter() {
            //println!("{:?}", &line);
            for i in 0..value / 2 {
                line.swap((start + i) % length, (start + value - 1 - i) % length)
            }
            start += (skip + value) % length;
            skip += 1;
        }
    }

    let block_size = 16;
    let dense: Vec<String> = line
        .chunks(block_size)
        .map(|chunk| chunk.iter().fold(0, |acc, &v| acc ^ v as u8))
        .map(|v| format!("{:x}", v))
        .collect();
    dense.join("")
}
