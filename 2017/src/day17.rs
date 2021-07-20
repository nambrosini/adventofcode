#[aoc_generator(day17)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
pub fn part1(input: &usize) -> usize {
    cycle(*input, 2017, 2017)
}

#[aoc(day17, part2)]
pub fn part2(input: &usize) -> usize {
    let mut curr_pos = 0;
    let mut val_at_pos_one = 0;

    for i in 1..50_000_001 {
        curr_pos = (curr_pos + input) % i + 1;
        if curr_pos == 1 {
            val_at_pos_one = i;
        }
    }

    val_at_pos_one
}

fn cycle(input: usize, n: usize, pos: usize) -> usize {
    let mut v = vec![0];
    let mut current_pos = 0;

    for i in 1..=n {
        let insert_pos = (current_pos + input) % v.len() + 1;

        if insert_pos > v.len() {
            v.push(i);
        } else {
            v.insert(insert_pos, i);
        }

        current_pos = insert_pos;
    }

    v[(v.iter().position(|&x| x == pos).unwrap() + 1) % v.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&3), 638);
    }
}
