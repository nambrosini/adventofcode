#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for i in input {
        if input.contains(&(2020 - i)) {
            return i * (2020 - i);
        }
    }

    unreachable!();
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for i in input {
        for j in input {
            if input.contains(&(2020 - i - j)) {
                return i * j * (2020 - i - j);
            }
        }
    }

    unreachable!();
}

#[test]
fn test1() {
    let s = "1721
979
366
299
675
1456";

    assert_eq!(part1(&generator(s)), 514579);
}

#[test]
fn sample1_part2() {
    let s = "1721
979
366
299
675
1456";

    assert_eq!(part2(&generator(s)), 241861950);
}
