use std::collections::HashSet;

#[aoc_generator(day03)]
pub fn generate(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day03, part1)]
pub fn part1(input: &[Vec<char>]) -> i32 {
    let mut sum = 0;
    let mut last_num = 0;
    for (i, l) in input.iter().enumerate() {
        for (j, e) in l.iter().enumerate() {
            if e.is_ascii_digit() {
                if check_adjacent_symbol(i, j, input) {
                    let v = get_value(j, l);
                    if v == last_num {
                        continue;
                    }
                    sum += v;
                    last_num = v;
                }
            } else {
                last_num = 0;
            }
        }
    }

    sum
}

#[aoc(day03, part2)]
pub fn part2(input: &[Vec<char>]) -> i32 {
    let mut res = 0;
    for (i, l) in input.iter().enumerate() {
        for (j, e) in l.iter().enumerate() {
            if e == &'*' {
                res += get_gears(i, j, input);
            }
        }
    }
    res
}

fn check_adjacent_symbol(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let x = x as i32;
    let y = y as i32;
    for i in -1..=1 {
        let new_x = x + i;
        if !(0..map.len() as i32).contains(&new_x) {
            continue;
        }
        for j in -1..=1 {
            let new_y = y + j;
            if i == 0 && j == 0 {
                continue;
            }
            if !(0..map[0].len() as i32).contains(&new_y) {
                continue;
            }
            let e = map[new_x as usize][new_y as usize];
            if !e.is_ascii_digit() && e != '.' {
                return true;
            }
        }
    }

    false
}

fn get_value(x: usize, row: &[char]) -> i32 {
    let mut start = x;
    let mut end = x;
    let mut found_start = false;
    let mut found_end = false;
    let mut i = 1;
    while !found_end || !found_start {
        if !found_end {
            if x + i < row.len() {
                if !row[x + i].is_ascii_digit() {
                    found_end = true;
                } else if row[x + i].is_ascii_digit() {
                    end = x + i;
                }
            } else {
                found_end = true;
            }
        }
        if !found_start {
            if x.checked_sub(i).is_some() {
                if !row[x - i].is_ascii_digit() {
                    found_start = true;
                } else {
                    start = x - i;
                }
            } else {
                found_start = true;
            }
        }
        i += 1;
    }

    row[start..=end].iter().collect::<String>().parse().unwrap()
}

fn get_gears(x: usize, y: usize, map: &[Vec<char>]) -> i32 {
    let mut gears = HashSet::new();
    let x = x as i32;
    let y = y as i32;
    for i in -1..=1 {
        let new_x = x + i;
        if !(0..map.len() as i32).contains(&new_x) {
            continue;
        }
        for j in -1..=1 {
            let new_y = y + j;
            if i == 0 && j == 0 {
                continue;
            }
            if !(0..map[0].len() as i32).contains(&new_y) {
                continue;
            }
            let e = map[new_x as usize][new_y as usize];
            if e.is_ascii_digit() {
                gears.insert(get_value(new_y as usize, &map[new_x as usize]));
                if gears.len() == 2 {
                    return gears.iter().product();
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_adjacent() {
        let s = "...
.63
.#.";
        let input: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        assert!(check_adjacent_symbol(1, 1, &input));
    }

    #[test]
    fn test_check_no_adjacent() {
        let s = "...
.63
...";
        let input: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        assert!(!check_adjacent_symbol(1, 1, &input));
    }

    #[test]
    fn test_find_value1() {
        let s = "...
.63
...";
        let input: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        assert_eq!(get_value(2, &input[1]), 63);
    }

    #[test]
    fn test_find_value2() {
        let s = "...
763
...";
        let input: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        assert_eq!(get_value(0, &input[1]), 763);
    }

    #[test]
    fn test_get_gears() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(get_gears(1, 1, &generate(input)), 16345);
    }

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(&generate(input)), 4361);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part2(&generate(input)), 467835);
    }
}
