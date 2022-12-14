type Lines = (usize, Vec<usize>);

fn egcd(first: i128, second: i128) -> (i128, i128, i128) {
    if first == 0 {
        (second, 0, 1)
    } else {
        let (g, x, y) = egcd(second % first, first);
        (g, y - (second / first) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> i128 {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        (x % n + n) % n
    } else {
        0
    }
}

#[aoc_generator(day13, part1)]
fn generator_part1(input: &str) -> Lines {
    let split: Vec<_> = input.lines().collect();

    let time: usize = split[0].parse().unwrap();

    let mut times: Vec<usize> = vec![];

    for s in split[1].split(',') {
        let time = s.parse();

        if let Ok(v) = time {
            times.push(v);
        }
    }

    (time, times)
}

#[aoc_generator(day13, part2)]
fn generator_part2(input: &str) -> Vec<(i128, i128)> {
    let lines = input.lines();

    lines
        .last()
        .unwrap()
        .trim()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| {
            let num = x.parse::<i128>().unwrap();
            ((num - (i as i128 % num)) % num, num)
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Lines) -> usize {
    let next_departing = input
        .1
        .iter()
        .enumerate()
        .map(|(i, x)| (i, input.0 + x - (input.0 % x)))
        .min_by_key(|(_, x)| *x)
        .unwrap();

    (next_departing.1 - input.0) * input.1[next_departing.0]
}

#[aoc(day13, part2)]
pub fn part2(input: &[(i128, i128)]) -> i128 {
    let n: i128 = input.iter().map(|(_, n_i)| n_i).product();

    let x: i128 = input
        .iter()
        .map(|(a_i, n_i)| {
            let tmp = n / n_i;
            a_i * mod_inv(tmp, *n_i) * tmp
        })
        .sum();

    x % n
}

#[test]
fn sample1() {
    let s = generator_part1(
        "939
7,13,x,x,59,x,31,19",
    );

    assert_eq!(part1(&s), 295);
}

#[test]
fn sample2() {
    let s = generator_part2(
        "939
17,x,13,19",
    );

    assert_eq!(part2(&s), 3417);
}

#[test]
fn sample3() {
    let s = generator_part2(
        "939
67,7,59,61",
    );

    assert_eq!(part2(&s), 754018);
}
