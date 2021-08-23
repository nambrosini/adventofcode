use itertools::Itertools;
    
#[aoc_generator(day21)]
pub fn generator(input: &str) -> Vec<Rule> {
    input.lines().map(|x| x.into()).collect_vec()
}

#[aoc(day21, part1)]
pub fn part1(input: &[Rule]) -> usize {
    let mut v = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#']
    ];

    for _ in 0..5 {
        let size = v.len();

        
    }

    v.iter().flatten().filter(|&&x| x == '#').count()
}

#[aoc(day21, part2)]
pub fn part2(input: &[Rule]) -> usize {
    0
}

pub struct Rule {
    from: Vec<Vec<char>>,
    to: Vec<Vec<char>>
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let split = s.split(" => ").collect_vec();

        let mut from = vec![];
        let mut v = vec![];

        for c in split[0].chars() {
            match c {
                '.' | '#' => v.push(c),
                '/' => {
                    from.push(v);
                    v = vec![];
                },
                _ => unreachable!()
            }
        }

        let mut to = vec![];

        for c in split[1].chars() {
            match c {
                '.' | '#' => v.push(c),
                '/' => {
                    to.push(v);
                    v = vec![];
                },
                _ => unreachable!()
            }
        }

        Self {
            from,
            to
        }
    }
}

impl Rule {
    fn check_pattern(&self, pattern: &[Vec<char>]) -> Option<Vec<Vec<char>>> {
        let r0h = Self::flip_h(pattern);
        let r0v = Self::flip_v(pattern);
        let r90 = Self::rotate(pattern);
        let r90h = Self::flip_h(&r90);
        let r90v = Self::flip_h(&r90);
        let r180 = Self::rotate(&r90);
        let r270 = Self::rotate(&r180);

        if pattern == self.from ||
            pattern == r0h ||
            pattern == r0v ||
            pattern == r90 ||
            pattern == r90h ||
            pattern == r90v ||
            pattern == r180 ||
            pattern == r270 {
                return Some(self.to.to_vec());
        }

        None
    }

    fn rotate(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let n = pattern.len();
        let mut ret = pattern.to_vec();

        for i in 0..n {
            for j in 0..n {
                ret[i][j] = pattern[n - j - 1][i];
            }
        }

        ret
    }

    fn flip_h(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut ret = pattern.to_vec();

        for row in 0..ret.len() / 2 {
            let temp = ret[ret.len() - row - 1].to_vec();
            ret[pattern.len() - row - 1] = pattern[row].to_vec();
            ret[row] = temp;
        }

        ret
    }

    fn flip_v(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut ret = pattern.to_vec();

        for row in 0..ret.len() {
            for col in 0..ret[0].len() / 2 {
                let temp = ret[row][ret[0].len() - col - 1];
                ret[row][pattern[0].len() - col - 1] = ret[row][col];
                ret[row][col] = temp;
            }
        }

        ret
    }
}