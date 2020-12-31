use serde_json::Value;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i64 {
    let input: Value = serde_json::from_str(input).unwrap();
    calc1(&input)
}

pub fn calc1(input: &Value) -> i64 {
    let mut sum: i64 = 0;
    match input {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(|value| calc1(value)).sum::<i64>(),
        Value::Object(object) => sum += object.values().map(|value| calc1(value)).sum::<i64>(),
        _ => (),
    }

    sum
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i64 {
    let input: Value = serde_json::from_str(input).unwrap();
    calc2(&input)
}

fn calc2(input: &Value) -> i64 {
    let mut sum: i64 = 0;

    match input {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(|value| calc2(value)).sum::<i64>(),
        Value::Object(object) => {
            let red_param_detected = object.values().any(|value| {
                if let Value::String(s) = value {
                    return s == "red";
                }
                false
            });
            if !red_param_detected {
                sum += object.values().map(|value| calc2(value)).sum::<i64>();
            }
        }
        _ => (),
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input1 = "[1,2,3]";
        let input2 = r#"{"a":2,"b":4}"#;

        assert_eq!(part1(&input1), 6);
        assert_eq!(part1(&input2), 6);
    }

    #[test]
    fn sample2_test2() {
        let input1 = "[1,2,3]";
        let input2 = r#"[1,{"c":"red","b":2},3]"#;
        let input3 = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let input4 = r#"[1,"red",5]"#;
        
        assert_eq!(part2(&input1), 6);
        assert_eq!(part2(&input2), 4);
        assert_eq!(part2(&input3), 0);
        assert_eq!(part2(&input4), 6);
    }
}