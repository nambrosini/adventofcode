pub fn solve_part_1(start: u32, end: u32) -> u32 {
    let mut vec = vec![];
    vec.extend(start..=end);

    vec.iter()
        .filter(|x| {
            let x = convert_to_vec(**x);

            check_double_digit(&x) && check_increasing(&x)
        })
        .count() as u32
}

pub fn solve_part_2(start: u32, end: u32) -> u32 {
    let mut vec = vec![];
    vec.extend(start..=end);

    vec.iter()
        .filter(|x| {
            let x = convert_to_vec(**x);

            check_double_digit_not_grouped(&x) && check_increasing(&x)
        })
        .count() as u32
}

fn convert_to_vec(password: u32) -> Vec<u32> {
    password
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

fn check_double_digit(password: &[u32]) -> bool {
    for i in 0..password.len() - 1 {
        if password[i] == password[i + 1] {
            return true;
        }
    }

    false
}

fn check_double_digit_not_grouped(password: &[u32]) -> bool {
    let mut count = 0;
    let mut last = 10;

    for i in password {
        if *i == last {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            last = *i;
            count = 1;
        }
    }

    count == 2
}

fn check_increasing(password: &[u32]) -> bool {
    for i in 0..password.len() - 1 {
        if password[i] > password[i + 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        assert_eq!(solve_part_1(111111, 111111), 1);
        assert_eq!(solve_part_1(223450, 223450), 0);
        assert_eq!(solve_part_1(123789, 123789), 0);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_part_2(112233, 112233), 1);
        assert_eq!(solve_part_2(123444, 123444), 0);
        assert_eq!(solve_part_2(111122, 111122), 1);
        assert_eq!(solve_part_2(111112, 111112), 0);
        assert!(check_double_digit_not_grouped(&vec![1, 1, 2, 2, 3, 3]));
    }
}
