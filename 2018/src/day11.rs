#[aoc_generator(day11)]
pub fn generator(input: &str) -> i32 {
    input.parse().unwrap()
}

#[aoc(day11, part1)]
pub fn part1(input: &i32) -> String {
    let mut max = i32::MIN;
    let mut cell: (i32, i32) = (0, 0);

    for i in 1..=300 - 2 {
        for j in 1..=300 - 2 {
            let mut sum = 0;
            for x in 0..3 {
                for y in 0..3 {
                    sum += get_fuel_level(i + x, j + y, *input);
                }
            }
            if sum > max {
                max = sum;
                cell = (i, j);
            }
        }
    }

    format!("{},{}", cell.0, cell.1)
}

#[aoc(day11, part2)]
pub fn part2(input: &i32) -> String {
    let mut max = i32::MIN;
    let mut cell: (i32, i32, i32) = (0, 0, 0);

    for i in 1..=300 {
        for j in 1..=300 {
            let mut sum = 0;
            for k in 1..=(300 - (i - 1)) {
                for x in 0..k {
                    for y in 0..k {
                        sum += get_fuel_level(i + x, j + y, *input);
                    }
                }
                if sum > max {
                    max = sum;
                    cell = (i, j, k);
                }
                if sum < 0 {
                    break;
                }
            }
        }
    }

    format!("{},{},{}", cell.0, cell.1, cell.2)
}

fn get_fuel_level(x: i32, y: i32, input: i32) -> i32 {
    let id = x + 10;
    let power_level = (id * y + input) * id;
    let power_level = power_level / 100 % 10;
    power_level - 5
}

#[test]
fn test() {
    assert_eq!(get_fuel_level(122, 79, 57), -5);
    assert_eq!(get_fuel_level(217,196, 39), 0);
    assert_eq!(get_fuel_level(101,153, 71), 4);
}