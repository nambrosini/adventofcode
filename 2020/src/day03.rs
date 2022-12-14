#[derive(Debug, Clone)]
struct Slope {
    down: usize,
    right: usize,
}

impl Slope {
    pub fn new(right: usize, down: usize) -> Self {
        Self { down, right }
    }

    pub fn count_trees(self, input: &[Vec<char>]) -> usize {
        let row_count = input[0].len();
        let mut x = 0;

        let mut count = 0;
        for y in (0..input.len()).step_by(self.down) {
            if input[y][x] == '#' {
                count += 1
            }

            x = (x + self.right) % row_count;
        }

        count
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    Slope::new(3, 1).count_trees(input)
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let slopes = vec![
        Slope::new(3, 1),
        Slope::new(1, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    slopes
        .iter()
        .fold(1, |res, x| res * x.clone().count_trees(input))
}

#[test]
fn test1() {
    let s = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    assert_eq!(part1(&generator(s)), 7);
}

#[test]
fn test2() {
    let s = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    assert_eq!(part2(&generator(s)), 336);
}
