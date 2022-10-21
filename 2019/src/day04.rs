#[aoc_generator(day04)]
pub fn generator(input: &str) -> (usize, usize) {
    let v: Vec<usize> = input.split('-').map(|x| x.parse().unwrap()).collect();
    (v[0], v[1])
}

#[aoc(day04, part1)]
pub fn part1(range: &(usize, usize)) -> usize {
    (range.0..=range.1)
        .filter(|x| filter(x))
        .count()
}

#[aoc(day04, part2)]
pub fn part2(range: &(usize, usize)) -> usize {
    (range.0..=range.1)
        .filter(|x| filter_2(x))
        .count()
}

pub fn filter(x: &usize) -> bool {
    let mut double = false;
    for i in (1..6).rev() {
        let a = x / 10usize.pow(i) % 10;
        let b = x / 10usize.pow(i - 1) % 10;
        if a == b {
            double = true;
        } else if a > b {
            return false;
        }
    }
    double
}

pub fn filter_2(x: &usize) -> bool {
    let mut v: [u8; 6] = [0; 6];

    for i in 0..6 {
        v[i] = (x / 10usize.pow(5 - i as u32) % 10) as u8;
    }

    let mut double = false;

    for i in 0..v.len() - 1  {
        let a = v[i];
        let b = v[i + 1];

        if a == b && !double {
            double = true;
            if let Some(x) = i.checked_sub(1) {
                let c = v[x];
                if c == a {
                    double = false;
                }
            }

            if i + 2 < 6 {
                let c = v[i + 2];
                if c == a {
                    double = false;
                }
            }
        }

        if a > b {
            return false;
        }
    }

    double
}

#[test]
pub fn test() {
    assert!(filter(&111111));
    assert!(!filter(&123450));
    assert!(!filter(&123456));
}

#[test]
pub fn test_2() {
    assert!(!filter_2(&123444));
    assert!(filter_2(&111122));
    assert!(filter_2(&112222));
}