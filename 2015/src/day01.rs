#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32{
    input.chars().fold(0, |sum, c| sum + match c {
        '(' => 1,
        ')' => -1,
        _ => unreachable!()
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut sum = 0;

    for (i, c) in input.chars().enumerate() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        };

        if sum == -1 {
            return i as i32 + 1;
        }
    }

    unreachable!();
}


#[cfg(test)]
mod tests {
    use super::*;

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}