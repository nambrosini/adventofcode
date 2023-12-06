pub struct Input {
    seeds: Vec<u64>,
    stages: Vec<Vec<[u64; 3]>>,
}

#[aoc_generator(day05)]
pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.split("\n\n").collect();
    // Getting the seed, skips the 'seeds:'
    let seeds = chunks[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    // Getting the stages. Each stage is in its own array, and has multiple ranges.
    let stages = chunks[1..]
        .iter()
        .map(|stage| {
            stage
                .split('\n')
                .skip(1)
                .map(|x| {
                    let y: Vec<u64> = x
                        .split_ascii_whitespace()
                        .map(|y| y.parse::<u64>().unwrap())
                        .collect();
                    [y[0], y[1], y[2]]
                })
                .collect()
        })
        .collect();

    Input { seeds, stages }
}

#[aoc(day05, part1)]
pub fn part1(input: &Input) -> u64 {
    let seeds = input.seeds.clone();
    map_seeds(&input.stages, &seeds)
}

fn map_seeds(stages: &[Vec<[u64; 3]>], seeds: &[u64]) -> u64 {
    let mut seeds = seeds.to_vec();
    for stage in stages {
        'outer: for seed in seeds.iter_mut() {
            for [dest, start, count] in stage {
                if *seed >= *start && *seed <= start + count {
                    *seed = *seed - start + dest;
                    continue 'outer;
                }
            }
        }
    }
    *seeds.iter().min().unwrap()
}

#[aoc(day05, part2)]
pub fn part2(input: &Input) -> u64 {
    let seeds: &mut Vec<u64> = &mut input
        .seeds
        .chunks(2)
        .flat_map(|seed| seed[0]..seed[0] + seed[1])
        .collect();

    map_seeds(&input.stages, seeds)
}

#[test]
fn test_part1() {
    let s = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(part1(&parse(s)), 35);
}

#[test]
fn test_part2() {
    let s = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(part2(&parse(s)), 46);
}
