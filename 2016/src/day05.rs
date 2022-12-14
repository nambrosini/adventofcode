use itertools::Itertools;

#[aoc(day05, part1)]
pub fn part1(input: &str) -> String {
    let mut s = String::new();

    for i in 0.. {
        print!("\r{}", i);
        let digest = md5::compute(format!("{}{}", input, i));
        let digest = format!("{:x}", digest);
        if &digest[..5] == "00000" {
            s.push_str(&digest[5..6]);
            println!("\n{}", s);

            if s.len() == 8 {
                return s;
            }
        }
    }

    unreachable!();
}

#[aoc(day05, part2)]
pub fn part2(input: &str) -> String {
    const VAL: String = String::new();
    let mut s: [String; 8] = [VAL; 8];

    for i in 0.. {
        // print!("\r{}", i);
        let digest = md5::compute(format!("{}{}", input, i));
        let digest = format!("{:x}", digest);
        if &digest[..5] == "00000" {
            let index: usize = if let Ok(v) = digest[5..6].parse::<usize>() {
                if v < s.len() {
                    v
                } else {
                    continue;
                }
            } else {
                continue;
            };

            if s[index] == String::new() {
                s[index] = digest[6..7].to_string();
            }

            if !s.iter().any(|c| c.is_empty()) {
                return s.iter().join("");
            }
        }
    }

    unreachable!();
}

// #[test]
// fn test1() {
//     let s = "abc";
//
//     assert_eq!(&part1(s), "18f47a30");
// }
//
// #[test]
// fn test2() {
//     let s = "abc";
//
//     assert_eq!(&part2(s), "05ace8e3");
// }
