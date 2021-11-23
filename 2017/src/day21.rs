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
        vec!['#', '#', '#'],
    ];

    for _ in 0..5 {
        let size = v.len();
        if size % 2 == 0 {
            v = take2(&v, input);
        } else {
            v = take3(&v, input);
        }
    }

    v.iter().flatten().filter(|&&x| x == '#').count()
}

fn take2(v: &[Vec<char>], rules: &[Rule]) -> Vec<Vec<char>> {
    let mut squares = vec![];

    for i in 0..(v.len() / 2) {
        for j in 0..(v.len() / 2) {
            let mut s = String::new();

            s.push(v[i * 2][j * 2]);
            s.push(v[i * 2 + 1][j * 2]);
            s.push('/');
            s.push(v[i * 2][j * 2 + 1]);
            s.push(v[i * 2 + 1][j * 2 + 1]);

            squares.push(s);
        }
    }

    'outer: for s in squares {
        for r in rules {
            if let Some(enhancement) = r.check_pattern(&string_to_vec(&s)) {
                s = enhancement;
                continue 'outer;
            }
        }
    }

    
}

fn string_to_vec(s: &str) -> Vec<Vec<char>> {
    let mut v = Vec::new();

    let split = s.split('/');

    for s in split {
        let mut v2 = Vec::new();
        for c in s.chars() {
            v2.push(c);
        }
        v.push(v2);
    }

    v
}

#[aoc(day21, part2)]
pub fn part2(input: &[Rule]) -> usize {
    0
}

pub struct Rule {
    from: String,
    to: String,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let split = s.split(" => ").collect_vec();

        Self { 
            from: split[0].to_string(), 
            to: split[1].to_string(),
        }
    }
}

impl Rule {
    fn check_pattern(&self, pattern: &[Vec<char>]) -> Option<String> {
        if self.from == Self::to_string(pattern) {
            return Some(self.to.clone());
        }
        
        let r0h = Self::flip_h(pattern);
        let r0v = Self::flip_v(pattern);
        let r90 = Self::rotate(pattern);
        let r90h = Self::flip_h(&r90);
        let r90v = Self::flip_h(&r90);
        let r180 = Self::rotate(&r90);
        let r270 = Self::rotate(&r180);

        if self.from == Self::to_string(&r0h) ||
            self.from == Self::to_string(&r0v) ||
            self.from == Self::to_string(&r90) ||
            self.from == Self::to_string(&r90h) ||
            self.from == Self::to_string(&r90v) ||
            self.from == Self::to_string(&r180) ||
            self.from == Self::to_string(&r270) {
            return Some(self.to.clone());
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

    fn to_string(chars: &[Vec<char>]) -> String {
        let mut s = String::new();

        for (i, r) in chars.iter().enumerate() {
            s.push_str(&r.iter().join(" "));
            if i < r.len() - 1 {
                s.push('/');
            }
        }

        s
    }
}