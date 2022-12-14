use itertools::Itertools;
use std::cmp;

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[usize]) -> usize {
    get_lowest_quantum(input, 3)
}

#[aoc(day24, part2)]
pub fn part2(input: &[usize]) -> usize {
    get_lowest_quantum(input, 4)
}

pub fn get_lowest_quantum(weights: &[usize], n: usize) -> usize {
    let expected_weight: usize = weights.iter().sum::<usize>() / n;
    let mut min = usize::MAX;
    let mut should_break = false;

    for i in 1..weights.len() {
        for group_weights in weights.iter().combinations(i) {
            println!("group_weights: {:?}", group_weights);
            if group_weights.iter().copied().sum::<usize>() == expected_weight {
                should_break = true;

                let quantum = group_weights.iter().copied().product();
                min = cmp::min(min, quantum);
            }
        }

        if should_break {
            break;
        }
    }

    min
}

#[test]
fn test1() {
    let s = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    assert_eq!(part1(&s), 99);
}

#[test]
fn test2() {
    let s = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    assert_eq!(part2(&s), 44);
}
