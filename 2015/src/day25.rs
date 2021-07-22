#[aoc_generator(day25)]
pub fn generator(input: &str) -> (usize, usize) {
    (2978, 3083)
}

#[aoc(day25, part1)]
pub fn part1(input: &(usize, usize)) -> i128 {
    let mut previous: i128 = 20151125;
    let mut index = (1, 1);

    while index != *input {
        index = get_next_diagonal(index);

        previous = (previous * 252533) % 33554393;
    }

    previous
}

fn get_next_diagonal(index: (usize, usize)) -> (usize, usize) {
    if index.0 == 1 { 
        (index.1 + 1, 1)
    } else {
        (index.0 - 1, index.1 + 1)
    }
}

#[test]
fn test1() {
    let i = (6, 6);

    assert_eq!(part1(&i), 27995004);
}