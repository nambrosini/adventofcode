#[aoc_generator(day20)]
pub fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
pub fn part1(input: &usize) -> usize {
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    for e in 2..boundary {
        for h in (e..boundary).step_by(e) {
            if h >= boundary {
                break;
            }

            houses[h] += e * 10;
            if houses[h] >= *input {
                boundary = h;
            }
        }
    }

    houses
        .iter()
        .enumerate()
        .filter(|&(_, h)| h >= input)
        .next()
        .map(|x| x.0)
        .unwrap()
}

#[aoc(day20, part2)]
pub fn part2(input: &usize) -> usize {
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    for e in 2..boundary {
        for (i, h) in (e..boundary).step_by(e).enumerate() {
            if i >= 50 || h >= boundary {
                break;
            }

            houses[h] += e * 11;
            if houses[h] >= *input {
                boundary = h;
            }
        }
    }

    houses
        .iter()
        .enumerate()
        .filter(|&(_, h)| h >= input)
        .next()
        .map(|x| x.0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let s = 130;

        assert_eq!(part1(&130), 9);
    }
}
