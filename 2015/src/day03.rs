use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[char]) -> usize {
    let mut pos = (0usize, 0usize);

    let mut visited: HashSet<(usize, usize)> = input
        .iter()
        .map(|next| {
            pos = match next {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                '^' => (pos.0, pos.1 + 1),
                'v' => (pos.0, pos.1 - 1),
                _ => unreachable!(),
            };

            pos
        })
        .collect();

    visited.insert((0, 0));
    visited.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &[char]) -> usize {
    let mut santa_pos = (0i32, 0i32);
    let mut robo_pos = (0i32, 0i32);

    let mut visited: HashSet<(i32, i32)> = input
        .iter()
        .enumerate()
        .map(|(i, next)| {
            let m = match next {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, 1),
                'v' => (0, -1),
                _ => unreachable!(),
            };

            if i % 2 == 1 {
                santa_pos = (santa_pos.0 + m.0, santa_pos.1 + m.1);
                santa_pos
            } else {
                robo_pos = (robo_pos.0 + m.0, robo_pos.1 + m.1);
                robo_pos
            }
        })
        .collect();

    visited.insert((0, 0));
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator(">");

        assert_eq!(part1(&s), 2);
    }

    #[test]
    fn sample2_test1() {
        let s = generator("^>v<");

        assert_eq!(part1(&s), 4);
    }

    #[test]
    fn sample3_test1() {
        let s = generator("^v^v^v^v^v");

        assert_eq!(part1(&s), 2);
    }

    #[test]
    fn sample1_test2() {
        let s = generator("^v");

        assert_eq!(part2(&s), 3);
    }

    #[test]
    fn sample2_test2() {
        let s = generator("^>v<");

        assert_eq!(part2(&s), 3);
    }

    #[test]
    fn sample3_test2() {
        let s = generator("^v^v^v^v^v");

        assert_eq!(part2(&s), 11);
    }
}
