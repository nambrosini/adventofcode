#[macro_use] extern crate lazy_static;
extern crate regex;

use std::cmp;

use regex::Regex;

pub fn solve_part_1(input: &[&str]) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<l>\d+)x(?P<w>\d+)x(?P<h>\d+)").unwrap();
    }

    input.iter()
        .map(|s| {
            let caps = RE.captures(s).unwrap();

            let l: u32 = caps["l"].parse().unwrap();
            let w: u32 = caps["w"].parse().unwrap();
            let h: u32 = caps["h"].parse().unwrap();

            let first = l * w;
            let second = w * h;
            let third = h * l;

            2 * (first + second + third) + cmp::min(first, cmp::min(second, third))
        }).sum()
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
}
