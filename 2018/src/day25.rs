#[aoc_generator(day25)]
pub fn generator(input: &str) -> Vec<Point> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day25, part1)]
pub fn part1(input: &[Point]) -> usize {
    let mut constellations: Vec<Vec<Point>> = vec![];

    for point in input {
        let indexes: Vec<usize> = constellations
            .iter()
            .enumerate()
            .filter(|(_, e)| e.iter().any(|p| point.calc_manhattan(p) <= 3))
            .map(|(i, _)| i)
            .collect();

        println!("{:?}", indexes);
        println!("{:?}", constellations.len());

        if indexes.is_empty() {
            constellations.push(vec![point.clone()]);
        } else {
            let target = indexes[0];
            constellations[target].push(point.clone());
            for i in indexes.iter().skip(1).rev() {
                let a = constellations.remove(*i);
                constellations[target].extend(a);
            }
        }
    }

    constellations.len()
}

#[derive(Debug, Clone)]
pub struct Point {
    coord: Vec<i32>,
}

impl Point {
    fn calc_manhattan(&self, other: &Self) -> u32 {
        let mut dist = 0;
        for (i, e) in self.coord.iter().enumerate() {
            dist += (*e - other.coord[i]).unsigned_abs();
        }
        dist
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let coord = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

        Self { coord }
    }
}

#[test]
fn test1() {
    let s = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
    let want = 2;
    let got = part1(&generator(s));

    assert_eq!(want, got)
}

#[test]
fn test2() {
    let s = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
    let want = 4;
    let got = part1(&generator(s));

    assert_eq!(want, got)
}

#[test]
fn test3() {
    let s = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
    let want = 3;
    let got = part1(&generator(s));

    assert_eq!(want, got)
}

#[test]
fn test4() {
    let s = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
    let want = 8;
    let got = part1(&generator(s));

    assert_eq!(want, got)
}
