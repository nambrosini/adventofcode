use itertools::Itertools;

#[aoc_generator(day09)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

#[aoc(day09, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if is_low_point(input, i, j) {
                sum += input[i][j] + 1;
            }
        }
    }

    sum
}

#[aoc(day09, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    let mut input = input.to_vec();

    let mut set: Vec<usize> = Vec::new();

    for j in 0..input[0].len() {
        for i in 0..input.len() {
            if input[i][j] != 9 {
                let size = find_basin(&mut input, i, j);
                set.push(size);
            }
        }
    }

    set.sort_unstable();

    set.iter().rev().take(3).product()
}

fn is_low_point(map: &[Vec<u32>], x: usize, y: usize) -> bool {
    let x = x as i32;
    let y = y as i32;

    let neighbours: [(i32, i32); 4]  = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for n in neighbours {
        let xx = x + n.0;
        let yy = y + n.1;

        if xx < 0 || yy < 0 || xx >= map.len() as i32 || yy >= map[0].len() as i32 {
            continue;
        }

        if map[x as usize][y as usize] >= map[xx as usize][yy as usize] {
            return false;
        }
    }

    true
}

fn get_neighbours(x: usize, y: usize, map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let x = x as i32;
    let y = y as i32;

    let neighbours: [(i32, i32); 4]  = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut v = vec![];

    for n in neighbours {
        let xx = x + n.0;
        let yy = y + n.1;

        if xx < 0 || yy < 0 || xx >= map.len() as i32 || yy >= map[0].len() as i32 {
            continue;
        }

        v.push((xx as usize, yy as usize));
    }

    v
}

fn find_basin(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let mut open_nodes = vec![(x, y)];
    let mut visited_nodes = vec![];

    while !open_nodes.is_empty() {
        let node = open_nodes.remove(0);

        if map[node.0][node.1] == 9 {
            continue;
        }

        visited_nodes.push(node);
        map[node.0][node.1] = 9;

        let neighbours = get_neighbours(node.0, node.1, map);

        for n in neighbours {
            if map[n.0][n.1] == 9 || visited_nodes.contains(&n) || open_nodes.contains(&n) {
                continue;
            }

            open_nodes.push(n);
        }
    }

    visited_nodes.len()
}

#[test]
fn test() {
    let s = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let s = generator(s);

    assert_eq!(part2(&s), 1134);
}