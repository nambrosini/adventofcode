use itertools::Itertools;

#[aoc_generator(day06)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect_vec()
}

#[aoc(day06, part1)]
pub fn part1(input: &[String]) -> String {
    let mut s = String::new();

    for i in 0..input[0].len() {
        let groups = input
            .iter()
            .flat_map(|l| l.chars().nth(i))
            .sorted()
            .group_by(|&x| x);

        let max: char = groups
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0;

        s.push(max);
    }

    s
}

#[aoc(day06, part2)]
pub fn part2(input: &[String]) -> String {
    let mut s = String::new();

    for i in 0..input[0].len() {
        let groups = input
            .iter()
            .flat_map(|l| l.chars().nth(i))
            .sorted()
            .group_by(|&x| x);

        let max: char = groups
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .min_by_key(|&(_, count)| count)
            .unwrap()
            .0;

        s.push(max);
    }

    s
}

#[test]
fn test1() {
    let s = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    let s = generator(s);

    assert_eq!(&part1(&s), "easter");
}

#[test]
fn test2() {
    let s = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    let s = generator(s);

    assert_eq!(&part2(&s), "advent");
}
