pub fn solve_part_1(input: &str) -> i32 {
    input.chars()
        .map(|c| {
            if c == '(' {
                1
            } else {
                -1
            }
        }).sum()
}

pub fn solve_part_2(input: &str) -> i32 {
    let iter = input.chars()
        .map(|c| {
            if c == '(' {
                1
            } else {
                -1
            }
        });

    let mut sum = 0;
    
    for (i, el) in iter.enumerate() {
        sum += el;

        if sum == -1 {
            return (i + 1) as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {

        assert_eq!(solve_part_1("(())"), 0);
        assert_eq!(solve_part_1("()()"), 0);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_part_1("((("), 3);
        assert_eq!(solve_part_1("(()(()("), 3);
        assert_eq!(solve_part_1("))((((("), 3);
    }

    #[test]
    fn test_three() {
        assert_eq!(solve_part_1("())"), -1);
        assert_eq!(solve_part_1("))("), -1);   
    }

    #[test]
    fn test_four() {
        assert_eq!(solve_part_1(")))"), -3);
        assert_eq!(solve_part_1(")())())"), -3);
    }

    #[test]
    fn test_five() {
        assert_eq!(solve_part_2(")"), 1);
        assert_eq!(solve_part_2("()())"), 5);
    }
}