#[macro_use] extern crate lazy_static;
extern crate regex;

use std::cmp;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<l>\d+)x(?P<w>\d+)x(?P<h>\d+)").unwrap();
}

fn get_sizes(input: &[&str]) -> Vec<(u32, u32, u32)> {
    input.iter()
        .map(|s| {
            let caps = RE.captures(s).unwrap();

            let l: u32 = caps["l"].parse().unwrap();
            let w: u32 = caps["w"].parse().unwrap();
            let h: u32 = caps["h"].parse().unwrap();

            (l, w, h)
        }).collect()
}

pub fn solve_part_1(input: &[&str]) -> u32 {
    let input = get_sizes(input);

    input.iter()
        .map(|(l, w, h)| {
            let first = l * w;
            let second = w * h;
            let third = h * l;

            2 * (first + second + third) + cmp::min(first, cmp::min(second, third))
        }).sum()
}

pub fn solve_part_2(input: &[&str]) -> u32 {
    let input = get_sizes(input);

    input.iter()
        .map(|(l, w, h)| {
            let (x, y) = get_smaller_pair(&vec![*l, *w, *h][..]);

            l * w * h + 2 * x + 2 * y
        }).sum()
}

fn get_smaller_pair(values: &[u32]) -> (u32, u32) {
    let mut x: (u32, u32) = (values[0], u32::max_value());

    for i in values.iter().skip(1) {
        if *i < x.0 {
            x.1 = x.0;
            x.0 = *i;
        } else if *i < x.1 {
            x.1 = *i;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec!["2x3x4"];

        assert_eq!(solve_part_1(&input), 58);
    }

    #[test]
    fn test_two() {
        let input = vec!["1x1x10"];

        assert_eq!(solve_part_1(&input), 43);
    }

    #[test]
    fn test_three() {
        let input = vec!["2x3x4"];

        assert_eq!(solve_part_2(&input), 34);   
    }

    #[test]
    fn test_four() {
        let input = vec!["1x1x10"];

        assert_eq!(solve_part_2(&input), 14);
    }
}
