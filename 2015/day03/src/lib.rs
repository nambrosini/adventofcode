use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> u32 {
    let santa: Vec<char> = input.chars().collect();
    visited_houses(&santa).len() as u32
}

fn visited_houses(input: &[char]) -> HashSet<(i32, i32)> {
    let v = input.iter().map(|c| match c {
        '^' => (0, 1),
        'v' => (0, -1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Unknown direction: {}", c),
    });

    let mut res = vec![(0, 0)];
    for i in v {
        let (x, y) = *res.last().unwrap();

        res.push((x + i.0, y + i.1));
    }

    let set: HashSet<(i32, i32)> = res.into_iter().collect();

    set
}

pub fn solve_part_2(input: &str) -> u32 {
    let santa: Vec<char> = input
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, e)| e)
        .collect();

    let robo_santa: Vec<char> = input
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, e)| e)
        .collect();

    let santa = visited_houses(&santa[..]);
    let robo_santa = visited_houses(&robo_santa[..]);

    let set: HashSet<&(i32, i32)> = santa.union(&robo_santa).collect();

    set.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_part_1(">"), 2);
        assert_eq!(solve_part_1("^>v<"), 4);
        assert_eq!(solve_part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_part_2("^v"), 3);
        assert_eq!(solve_part_2("^>v<"), 3);
        assert_eq!(solve_part_2("^v^v^v^v^v"), 11);
    }
}
