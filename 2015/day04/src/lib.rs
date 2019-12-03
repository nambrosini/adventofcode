pub fn solve_part_1(input: &str) -> u32 {
    find_hash(input, "00000")
}

pub fn solve_part_2(input: &str) -> u32 {
    find_hash(input, "000000")
}

fn find_hash(input: &str, start: &str) -> u32 {
    let mut counter = 0;
    let mut digest = md5::compute(format!("{}{}", &input, counter.to_string()));

    while &format!("{:?}", digest)[..start.len()] != start {
        counter += 1;
        digest = md5::compute(format!("{}{}", &input, counter));
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve_part_1("abcdef"), 609043);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve_part_1("pqrstuv"), 1048970);
    }
}
