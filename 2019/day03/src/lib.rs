use std::collections::{HashSet, HashMap};

enum Dir {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl Dir {
    fn new(input: &str) -> Self {
        let dir = &input[..1];
        let len = input[1..].parse().unwrap();

        match dir {
            "U" => Dir::Up(len),
            "D" => Dir::Down(len),
            "L" => Dir::Left(len),
            "R" => Dir::Right(len),
            _ => panic!("Direction unknown: {}", dir)
        }
    }
}

pub struct Wire {
    path: Vec<(i64, i64)>
}

impl Wire {
    pub fn new(input: &[&str]) -> Wire {
        let path: Vec<Dir> = input.iter()
            .map(|x| Dir::new(x))
            .collect();


        let mut res: Vec<(i64, i64)> = vec![(0, 0)];

        for p in path {
            let (len, (dx, dy)) = match p {
                Dir::Up(v) => (v, (0, 1)),
                Dir::Down(v) => (v, (0, -1)),
                Dir::Left(v) => (v, (-1, 0)),
                Dir::Right(v) => (v, (1, 0))
            };

            let (x, y) = *res.last().unwrap();

            res.extend((1..=(len as i64)).map(|d| (x + dx * d, y + dy * d)));
        }

        Wire {
            path: res
        }
    }

    pub fn manhattan(&self, other: &Wire) -> i64 {
        let path1: HashSet<(i64, i64)> = self.path.clone().into_iter().skip(1).collect();
        let path2: HashSet<(i64, i64)> = other.path.clone().into_iter().skip(1).collect();
        let common_points: Vec<&(i64, i64)> = path1
            .iter()
            .skip(1)
            .filter(|e| path2.contains(&e))
            .collect();

        common_points.iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap()
    }

    pub fn steps(&self, other: &Wire) -> i64 {
        let path1: HashMap<(i64, i64), usize> = self.path
            .iter()
            .enumerate()
            .map(|(i, (x, y))| ((*x, *y), i))
            .rev()
            .collect();

        let common_points = other.path
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(d2, (x, y))| {
                let d1 = path1.get(&(*x, *y))?;
                Some(d1 + d2)
            });

        common_points.min().unwrap() as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_one() {
        let content = fs::read_to_string("test1.txt").unwrap();

        let lines: Vec<Vec<&str>> = content.lines().map(|l| l.split(",").collect()).collect();

        let wire1 = Wire::new(&lines[0]);
        let wire2 = Wire::new(&lines[1]);

        assert_eq!(wire1.manhattan(&wire2), 6);
        assert_eq!(wire1.steps(&wire2), 30);
    }

    #[test]
    fn test_two() {
        let content = fs::read_to_string("test2.txt").unwrap();

        let lines: Vec<Vec<&str>> = content.lines().map(|l| l.split(",").collect()).collect();

        let wire1 = Wire::new(&lines[0]);
        let wire2 = Wire::new(&lines[1]);

        assert_eq!(wire1.manhattan(&wire2), 159);
        assert_eq!(wire1.steps(&wire2), 610);
    }

    #[test]
    fn test_three() {
        let content = fs::read_to_string("test3.txt").unwrap();

        let lines: Vec<Vec<&str>> = content.lines().map(|l| l.split(",").collect()).collect();

        let wire1 = Wire::new(&lines[0]);
        let wire2 = Wire::new(&lines[1]);

        assert_eq!(wire1.manhattan(&wire2), 135);
        assert_eq!(wire1.steps(&wire2), 410);
    }
}