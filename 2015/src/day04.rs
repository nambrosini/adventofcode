use md5::{Digest, Md5};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let key = input.as_bytes();
    for i in 0..std::usize::MAX {
        let mut hasher = Md5::new();
        hasher.update(key);
        hasher.update(i.to_string().as_bytes());

        let output = hasher.finalize();

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            return i as usize;
        }
    }

    unreachable!();
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let key = input.as_bytes();
    for i in 0..std::usize::MAX {
        let mut hasher = Md5::new();
        hasher.update(key);
        hasher.update(i.to_string().as_bytes());

        let output = hasher.finalize();

        let first_five = output[0] as i32 + output[1] as i32 + output[2] as i32;

        if first_five == 0 {
            return i as usize;
        }
    }

    unreachable!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn sample1_test1() {
//         let s = "abcdef";

//         assert_eq!(part1(s), 609043);
//     }

//     #[test]
//     fn sample2_test1() {
//         let s = "pqrstuv";

//         assert_eq!(part1(s), 1048970);
//     }
// }
