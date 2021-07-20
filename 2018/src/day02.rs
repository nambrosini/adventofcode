use itertools::Itertools;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect_vec()
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> usize {
    let checked = input.iter().map(|x| check_letters(x)).collect_vec();

    let two = checked.iter().fold(0, |sum, x| sum + x.0);
    let three = checked.iter().fold(0, |sum, x| sum + x.1);

    two * three
}

fn check_letters(id: &str) -> (usize, usize) {
    let mut vec = vec![];

    let mut two = 0;
    let mut three = 0;

    let v = id.chars().collect_vec();

    for c in v.iter() {
        if !vec.contains(c) {
            let count = v.iter().filter(|&x| x == c).count();

            if count == 2 && two == 0 {
                two += 1;
            } else if count == 3 && three == 0 {
                three += 1;
            }

            vec.push(*c);
        }
    }

    (two, three)
}

#[aoc(day2, part2)]
pub fn part2(id: &[String]) -> String {
    let (i1, i2) = find_similar(id);

    let mut vec = vec![];

    let id1 = id[i1].chars();
    let mut id2 = id[i2].chars();

    for c in id1 {
        if c == id2.next().unwrap() {
            vec.push(c);
        }
    }

    vec.iter().join("")
}

fn find_similar(id: &[String]) -> (usize, usize) {
    for i in 0..id.len() {
        for j in 0..id.len() {
            let b1 = id[i].chars().collect_vec();
            let b2 = id[j].chars().collect_vec();

            let mut diff = 0;

            'inner: for k in 0..b1.len() {
                if b1[k] != b2[k] {
                    if diff == 0 {
                        diff = 1;
                    } else {
                        diff = 2;
                        break 'inner;
                    }
                }
            }

            if diff == 1 {
                return (i, j);
            }
        }
    }

    unreachable!();
}

#[test]
fn test_1() {
    let s = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    let s = generator(&s);

    assert_eq!(part1(&s), 12);
}
