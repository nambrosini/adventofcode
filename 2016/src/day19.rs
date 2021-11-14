#[aoc_generator(day19)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day19, part1)]
pub fn part1(input: &usize) -> usize {
    winner(*input)
}

#[aoc(day19, part2)]
pub fn part2(input: &usize) -> usize {
    winner2(*input)
}

fn next_power_of_three(mut n: usize) -> usize {
    let mut result = 1;
    while n > 0 {
        n /= 3;
        result *= 3;
    }
    result
}

fn winner2(elves: usize) -> usize {
    let next = next_power_of_three(elves);
    let prev = next / 3;
    if elves == prev {
        elves
    } else if elves > prev * 2 {
        elves * 2 - next
    } else {
        elves - prev
    }
}

fn winner(elves: usize) -> usize {
    if elves.is_power_of_two() {
        1
    } else {
        ((elves << 1) ^ elves.next_power_of_two()) | 1
    }
}


#[test]
fn test() {
    assert_eq!(part1(&5), 3);
}

#[test]
fn test2() {
    assert_eq!(part2(&5), 2);
}