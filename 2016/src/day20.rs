use itertools::Itertools;

#[aoc_generator(day20)]
fn generator(input: &str) -> Vec<(usize, usize)> {
    input.lines()
        .map(|line| line.split('-').collect_vec())
        .map(|l| (l[0].parse().unwrap(), l[1].parse().unwrap()))
        .collect_vec()
}

#[aoc(day20, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    let mut index = 0;

    loop {
        let mut blocked = false;
        for (start, end) in input {
            if index >= *start && index <= *end {
                blocked = true;
                break;
            }
        }
        if !blocked {
            return index;
        }
        index += 1;
    }
}

#[aoc(day20, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    const MAX: usize = 4294967295;

    let mut input = input.to_vec();
    input.sort_unstable();

    println!("{:?}", input.last());

    let mut input_iter = input.iter();
    
    let mut count = 0;
    let mut index = 0;
    let mut next = input_iter.next();

    loop {
        if let Some(next_u) = next {
            if index < next_u.0 {
                count += next_u.0 - index - 1;
                index = next_u.1;
            } else if index >= next_u.0 && index <= next_u.1 {
                index = next_u.1;
            }
            println!("{:?}, {}", next_u, index);
            next = input_iter.next();
        } else {
            println!("{}, {}", MAX, index);
            count += MAX - index;
            break;
        }
    }

    count
}

#[test]
fn test2() {
    let s = generator(&std::fs::read_to_string("input/2016/day20.txt").unwrap());

    assert_eq!(part2(&s), 200);
}