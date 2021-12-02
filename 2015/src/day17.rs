use itertools::Itertools;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect_vec()
}

#[aoc(day17, part1)]
pub fn part1(input: &[usize]) -> usize {
    let v = get_combinations(input);
    v.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &[usize]) -> usize {
    let v = get_combinations(input);
    let min = v.iter().map(|x| x.len()).min().unwrap();
    v.iter().filter(|x| x.len() == min).count()
}

fn get_combinations(input: &[usize]) -> Vec<Vec<&usize>> {
    let mut v: Vec<Vec<Vec<&usize>>> = vec![];
    for i in 1..input.len() {
        v.push(
            input
                .iter()
                .combinations(i)
                .filter(|x| x.iter().copied().sum::<usize>() == 150)
                .collect_vec(),
        );
    }

    v.iter().flatten().cloned().collect_vec()
}
