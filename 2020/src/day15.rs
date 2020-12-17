use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<usize> {
    input.split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut map: Vec<usize> = input.clone().to_vec();

    while map.len() < 2020 {
        let filter: Vec<_> = map.iter()
            .enumerate()
            .filter(|(_, &x)| x == *map.last().unwrap())
            .map(|(i, _)| i)
            .collect();

        if filter.len() > 1 {
            let val = filter[filter.len() - 1] - filter[filter.len() - 2];
            map.push(val);
        } else {
            map.push(0);
        }
    }

    *map.last().unwrap()
}

#[aoc(day15, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut map: HashMap<usize, (Option<usize>, usize)> = HashMap::new();

    for i in 0..input.len() {
        map.insert(input[i], (None, i));
    }

    let mut last_number = *input.last().unwrap();
    let mut counter = input.len();

    while counter < 30000000 {
        let last = map[&last_number];

        if last.0 == None {
            let new = map.entry(0).or_insert((None, counter));
            *new = (Some(new.1), counter);
            last_number = 0;
        } else {
            last_number = last.1 - last.0.unwrap();
            let new = map.entry(last_number).or_insert((None, counter));
            *new = (Some(new.1), counter);
        }

        counter += 1;
    }

    last_number
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_test1() {
        let s = generator("1,3,2");

        assert_eq!(part1(&s), 1);
    }

    #[test]
    fn sample2_test1() {
        let s = generator("2,1,3");

        assert_eq!(part1(&s), 10);
    }

    #[test]
    fn sample3_test1() {
        let s = generator("1,2,3");

        assert_eq!(part1(&s), 27);
    }

    #[test]
    fn sample4_test1() {
        let s = generator("2,3,1");

        assert_eq!(part1(&s), 78);
    }

    #[test]
    fn sample5_test1() {
        let s = generator("3,2,1");

        assert_eq!(part1(&s), 438);
    }

    #[test]
    fn sample6_test1() {
        let s = generator("3,1,2");

        assert_eq!(part1(&s), 1836);
    }

    // #[test]
    // fn sample1_test2() {
    //     let s = generator("1,3,2");

    //     assert_eq!(part2(&s), 2578);
    // }
}