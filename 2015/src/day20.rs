#[aoc_generator(day20)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
pub fn part1(input: &usize) -> usize {
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    let mut e = 2;

    while e < boundary {
        for h in (e..boundary).step_by(e) {
            if h >= boundary {
                break;
            }

            houses[h] += e * 10;
            if houses[h] >= *input {
                boundary = h;
            }
        }

        e += 1;
    }

    houses
        .iter()
        .enumerate()
        .find(|&(_, h)| h >= input)
        .unwrap()
        .0
}

#[aoc(day20, part2)]
pub fn part2(input: &usize) -> usize {
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    let mut e = 2;

    while e < boundary {
        for (i, h) in (e..boundary).step_by(e).enumerate() {
            if i >= 50 || h >= boundary {
                break;
            }

            houses[h] += e * 11;
            if houses[h] >= *input {
                boundary = h;
            }
        }
        e += 1;
    }

    houses
        .iter()
        .enumerate()
        .find(|&(_, h)| h >= input)
        .unwrap()
        .0
}
