use itertools::Itertools;

#[aoc_generator(day07)]
pub fn generator(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| {
            l.replace('[', " ")
                .replace("]", " ")
                .split(" ")
                .map(|x| x.to_string())
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|p| {
            (
                p.iter().step_by(2).join(" "),
                p.iter().skip(1).step_by(2).join(" "),
            )
        })
        .filter(|(x, y)| abba(x) && !abba(y))
        .count()
}

#[aoc(day07, part2)]
pub fn part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|p| {
            (
                p.iter().step_by(2).join(" "),
                p.iter().skip(1).step_by(2).join(" "),
            )
        })
        .filter(|(x, y)| {
            x.chars()
                .zip(x.chars().skip(1))
                .zip(x.chars().skip(2))
                .map(|((c0, c1), c2)| (c0, c1, c2))
                .any(|(c0, c1, c2)| {
                    c0 == c2 && c0 != c1 && y.contains(&format!("{}{}{}", c1, c0, c1))
                })
        })
        .count()
}

fn abba(s: &str) -> bool {
    let zips = s
        .chars()
        .zip(s.chars().skip(1))
        .zip(s.chars().skip(2))
        .zip(s.chars().skip(3))
        .map(|(((c0, c1), c2), c3)| (c0, c1, c2, c3))
        .collect_vec();

    zips.iter()
        .any(|(c0, c1, c2, c3)| c0 == c3 && c1 == c2 && c0 != c1)
}

#[test]
pub fn test1() {
    let s = "abbaasdf[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

    let s = generator(s);

    assert_eq!(part1(&s), 2);
}
