use std::collections::HashMap;

#[aoc_generator(day17)]
pub fn generator(input: &str) -> HashMap<(usize, usize), char> {
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    map.insert((500, 0), '+');

    for l in input.lines() {
        let split: Vec<&str> = l.split(", ").collect();
        let (x_min, x_max, y_min, y_max): (usize, usize, usize, usize) = if split[0].starts_with('x') {
            let x = split[0].split('=').last().unwrap().parse().unwrap();

            let y = split[1].split('=').last().unwrap();
            let y: Vec<&str> = y.split("..").collect();

            (x, x, y[0].parse().unwrap(), y[1].parse().unwrap())
        } else {
            let y = split[0].split('=').last().unwrap().parse().unwrap();

            let x = split[1].split('=').last().unwrap();
            let x: Vec<&str> = x.split("..").collect();

            (x[0].parse().unwrap(), x[1].parse().unwrap(), y, y)
        };

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                map.insert((x, y), '#');
            }
        }
    }

    map
}

#[aoc(day17, part1)]
pub fn part1(map: &HashMap<(usize, usize), char>) -> usize {
    let mut map = map.clone();
    let mut current = (500, 0);
    let mut next = (500, 1);

    for _ in 0..20 {
        if let Some(x) = map.get(&next) {
            if x == &'#' || x == &'~' {
                map.insert(current, '~');
                for y in 1.. {
                    if let Some(v) = map.get(&(current.0 + y, current.1)) {
                        if v == &'#' {
                            break;
                        }
                    } else {
                        map.insert((current.0 + y, current.1), '~');
                    }
                }

                for y in 1.. {
                    if let Some(v) = map.get(&(current.0 - y, current.1)) {
                        if v == &'#' {
                            break;
                        }
                    } else {
                        map.insert((current.0 - y, current.1), '~');
                    }
                }

                current = (current.0, current.1 - 1);
                next = (next.0, next.1 - 1);
            }
        } else {
            map.insert(next, '|');
            current = next;
            next = (next.0, next.1 + 1);
        }
        print_map(&map);
    }
    0
}

fn print_map(map: &HashMap<(usize, usize), char>) {
    let min_x = map.keys().min_by_key(|k| k.0).unwrap().0;
    let max_x = map.keys().max_by_key(|k| k.0).unwrap().0;
    let min_y = map.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = map.keys().max_by_key(|k| k.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let v = if let Some(v) = map.get( &(x, y)) {
                *v
            } else {
                '.'
            };
            print!("{}", v);
        }
        println!()
    }
    println!()
}


#[test]
pub fn test() {
    let s = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
    assert_eq!(57, part1(&generator(s)));
}