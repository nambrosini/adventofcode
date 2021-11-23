use regex::Regex;

#[aoc_generator(day09)]
pub fn generator(input: &str) -> (usize, usize) {
    let re = Regex::new(r"(\d+)[\sa-z;]+(\d+)").unwrap();

    let caps = re.captures_iter(input).next().unwrap();

    (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap())
}

#[aoc(day09, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    solve(input)
}

#[aoc(day09, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    solve(&(input.0, input.1 * 100))
}

fn solve(input: &(usize, usize)) -> usize {
    let mut players = vec![0; input.1];

    let mut played = vec![0];
    let mut current_index: usize = 0;
    let mut current_played = 1;

    while current_played <= input.1 {
        println!("{}", current_played);
        if current_played % 23 == 0 {
            let removed_marble_index = if let Some(sub) = current_index.checked_sub(7) {
                sub
            } else {
                played.len() - (7 - current_index)
            };
            let removed_marble = played.remove(removed_marble_index);
            players[current_played % input.0] += current_played + removed_marble;
            current_index = removed_marble_index
        } else {
            let next_index = (current_index + 1) % played.len();
            if next_index == played.len() {
                played.push(current_played);
            } else {
                played.insert(next_index + 1, current_played);
            }
            current_index = next_index + 1;
        }
        current_played += 1;
    }

    *players.iter().max().unwrap()
}

#[test]
fn test10() {
    let s = generator("9 players; last marble is worth 32 points");
    assert_eq!(part1(&s), 32);
}

#[test]
fn test11() {
    let s = generator("10 players; last marble is worth 1618 points");
    assert_eq!(part1(&s), 8317);
}

#[test]
fn test12() {
    let s = generator("13 players; last marble is worth 7999 points");
    assert_eq!(part1(&s), 146373);
}

#[test]
fn test13() {
    let s = generator("17 players; last marble is worth 1104 points");
    assert_eq!(part1(&s), 2764);
}

#[test]
fn test14() {
    let s = generator("21 players; last marble is worth 6111 points");
    assert_eq!(part1(&s), 54718);
}

#[test]
fn test15() {
    let s = generator("30 players; last marble is worth 5807 points");
    assert_eq!(part1(&s), 37305);
}