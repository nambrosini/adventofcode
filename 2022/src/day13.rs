use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Vec<Packet> {
    input
        .split("\n\n")
        .flat_map(|s| {
            let split: Vec<&str> = s.lines().collect();
            vec![
                parse_packet(split[0]).unwrap().1,
                parse_packet(split[1]).unwrap().1,
            ]
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(packets: &[Packet]) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, v)| v[0] < v[1])
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(packets: &[Packet]) -> usize {
    let mut packets = packets.to_vec();

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.extend_from_slice(&[div1.clone(), div2.clone()]);

    packets
        .iter()
        .sorted()
        .enumerate()
        .filter(|(_, p)| p == &&div1 || p == &&div2)
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Debug, Clone)]
pub enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

fn parse_packet(value: &str) -> IResult<&str, Packet> {
    alt((
        map(i32, Packet::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
    ))(value)
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(_), Packet::Int(_)) => {
                self.cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).cmp(other)
            }
            (Packet::List(l), Packet::List(r)) => {
                for (e1, e2) in l.iter().zip(r) {
                    if let Some(res) = e1.partial_cmp(e2) {
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

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
