pub fn solve_part_1(start: u32, end: u32) -> u32 {
    let mut vec = vec![];
    vec.extend(start..=end);

    vec.iter()
        .filter(|x| check_valid_1(&convert_to_vec(**x)))
        .count() as u32
}

pub fn solve_part_2(start: u32, end: u32) -> u32 {
    let mut vec = vec![];
    vec.extend(start..=end);

    vec.iter()
        .filter(|x| check_valid_2(&convert_to_vec(**x)))
        .count() as u32
}

fn convert_to_vec(mut password: u32) -> [u8; 6] {
    let mut res: [u8; 6] = [0; 6];

    for i in res.iter_mut().rev() {
        *i = (password % 10) as u8;
        password /= 10;
    }

    res
}

fn check_valid_1(password: &[u8; 6]) -> bool {
    let mut double = false;

    for i in 0..password.len() - 1 {
        if password[i] <= password[i + 1] {
            if !double {
                double = password[i] == password[i + 1];
            }
        } else {
            return false;
        }
    }

    double
}

fn check_valid_2(password: &[u8; 6]) -> bool {
    let mut count = 0;
    let mut last = 10;

    for i in 0..password.len() {
        if i == password.len() - 1 || password[i] <= password[i + 1] {
            if password[i] == last {
                count += 1;
            } else if count == 2 {
                last = 10;
            } else {
                count = 1;
                last = password[i];
            }
        } else {
            return false;
        }
    }

    count == 2
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
    }
}
