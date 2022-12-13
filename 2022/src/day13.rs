use std::cmp::Ordering;
use std::vec;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::delimited;

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<Packet> {
    input.split("\n\n")
        .flat_map(|l| {
            let mut split = l.lines();
            vec![parse_packet(split.next().unwrap()).unwrap().1, parse_packet(split.next().unwrap()).unwrap().1]
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Packet]) -> usize {
    let mut res = 0;
    for i in (0..input.len()).step_by(2) {
        if input[i] < input[i + 1] {
            res += i / 2 + 1;
        }
    }
    res
}

#[aoc(day13, part2)]
fn part2(input: &[Packet]) -> usize {
    let mut packets = input.to_vec();
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();

    let p1 = packets.iter().position(|e| e == &div1).unwrap() + 1;
    let p2 = packets.iter().position(|e| e == &div2).unwrap() + 1;
    p1 * p2
}

#[derive(Debug, Clone)]
pub enum Packet {
    Int(i64),
    List(Vec<Packet>)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(i64, Packet::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        )
    ))(input)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.partial_cmp(r),
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            },
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            },
            (Packet::List(l), Packet::List(r)) => {
                for (e1, e2) in l.iter().zip(r) {
                    if let Some(res) = e1.partial_cmp(e2) {
                        if res != Ordering::Equal {
                            return Some(res)
                        }
                    }
                }
                l.len().partial_cmp(&r.len())
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Packet {}

#[test]
fn test() {
    let s = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(13, part1(&generator(s)));
}

#[test]
fn test2() {
    let s = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(140, part2(&generator(s)));
}