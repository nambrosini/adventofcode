use regex::Regex;

#[aoc_generator(day09)]
pub fn generator(input: &str) -> (usize, usize) {
    let re = Regex::new(r"(?P<players>\d+)[\sa-z;]+(?P<points>\d+)").unwrap();

    let caps = re.captures(input).unwrap();

    let players = caps["players"].parse().unwrap();
    let points = caps["points"].parse().unwrap();

    (players, points)
}

#[aoc(day09, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    solve(input.0, input.1)
}

#[aoc(day09, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    solve(input.0, input.1 * 100)
}

fn solve(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];

    let mut played = vec![0];
    let mut current_index: usize = 0;
    let mut current_played = 1;

    while current_played <= last_marble {
        print!("\r{}", current_played);
        if current_played % 23 == 0 {
            let removed_marble_index = if let Some(sub) = current_index.checked_sub(7) {
                sub
            } else {
                played.len() - (7 - current_index)
            };
            let removed_marble = played.remove(removed_marble_index);
            scores[current_played % players] += current_played + removed_marble;
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

    *scores.iter().max().unwrap()
}

#[test]
fn test() {
    assert_eq!(32, part1(&(9, 25)));
    assert_eq!(8317, part1(&(10, 1618)));
    assert_eq!(146373, part1(&(13, 7999)));
    assert_eq!(2764, part1(&(17, 1104)));
    assert_eq!(54718, part1(&(21, 6111)));
    assert_eq!(37305, part1(&(30, 5807)));
}