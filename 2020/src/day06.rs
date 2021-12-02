use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_owned()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| s.replace("\n", "").chars().collect::<HashSet<char>>().len())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[String]) -> usize {
    let input: Vec<Vec<Vec<char>>> = input
        .iter()
        .map(|s| s.split('\n').map(|x| x.chars().collect()).collect())
        .collect();

    let mut sum = 0;

    for group in input {
        let mut s = String::new();
        let first_passenger = &group[0];

        for letter in first_passenger {
            let mut letter_all = true;
            for i in group.iter().skip(1) {
                if !i.contains(letter) {
                    letter_all = false;
                }
            }
            if letter_all {
                s.push(*letter);
            }
        }
        sum += s.len();
    }

    sum
}


#[test]
fn sample1_part1() {
    let s = generator("abc

a
b
c

ab
ac

a
a
a
a

b");

    assert_eq!(part1(&s), 11);
}

#[test]
fn sample1_part2() {
    let s = generator("abc

a
b
c

ab
ac

a
a
a
a

b");

    assert_eq!(part2(&s), 6);
}
