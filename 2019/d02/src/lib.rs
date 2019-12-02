use std::fs;
use std::io;

pub fn read_and_convert(filename: &str) -> Result<Vec<i32>, io::Error> {
    let content = fs::read_to_string(filename)?;
    let lines = content.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    Ok(lines)
} 

pub fn calculate_fuel_weight(parts: &[i32]) -> i32 {
    let mut sum: i32 = 0;

    for i in parts {
        let mut n: i32 = i / 3 - 2;
        while n > 0 {
            sum += n;
            n = n / 3 - 2;
        }
    }

    sum
}