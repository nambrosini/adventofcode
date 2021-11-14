use itertools::Itertools;

#[aoc_generator(day19)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day19, part1)]
pub fn part1(input: &usize) -> usize {
    // solve(*input, &take_next_present)
    solve(*input)
}

#[aoc(day19, part2)]
pub fn part2(input: &usize) -> usize {
    solve2(*input)
}

fn solve2(input: usize) -> usize {
    let mut elves: Vec<(usize, usize)> = Vec::new();

    for i in 1..=input {
        elves.push((i, 0));
    }

    let mut index = 0;

    while elves.len() > 1 {
        let next_index = (index + elves.len() / 2) % elves.len();
        index = (index + 1) % elves.len();
        elves.remove(next_index);
    }

    elves[0].0
}

// fn take_present_in_front(elves: &mut Vec<(usize, usize)>, index: usize) {
//     let next_index = (index + elves.len() / 2) % elves.len();

//     elves.remove(next_index);
// }

fn solve(input: usize) -> usize {
    let mut elves: Vec<usize> = (1..=input).collect_vec();

    let mut index = 0;

    while elves.len() > 1 {
        let next_index = (index + 1) % elves.len();
        elves.remove(next_index);
        index = next_index;
    }

    elves[0]
}

#[test]
fn test() {
    assert_eq!(part1(&5), 3);
}

#[test]
fn test2() {
    assert_eq!(part2(&5), 2);
}