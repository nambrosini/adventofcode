use itertools::Itertools as _;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect_vec()
}

#[aoc(day8, part1)]
pub fn part1(input: &[String]) -> usize {
    let mut delta = 0;
    for s in input.iter() {
        // decode each line in the input and keep a running some of the difference.
        let literal_length = s.len();
        let decoded_length = decode(s).len();
        delta += literal_length - decoded_length;
    }
    delta
}

// Implement a decoder. It's not stricly necessary, but it's one way to solve things.
fn decode(input: &str) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    let mut i = 1;
    while i < input.len() - 1 {
        if input[i] == b'\\' {
            if input[i + 1] == b'\\' {
                r.push('\\');
                i += 2;
            } else if input[i + 1] == b'"' {
                r.push('"');
                i += 2;
            } else if input[i + 1] == b'x' {
                r.push('?'); // I'm being lazy!
                i += 4;
            }
        } else {
            r.push(input[i] as char);
            i += 1;
        }
    }
    r
}

fn encode(input: &str) -> String {
    let input = input.as_bytes();
    let mut r = String::new();
    r.push('"');
    for c in input.iter() {
        match *c {
            b'"' => r.push_str("\\\""),
            b'\\' => r.push_str("\\\\"),
            _ => r.push(*c as char),
        }
    }
    r.push('"');
    r
}

#[aoc(day8, part2)]
pub fn part2(input: &[String]) -> usize {
    let mut delta = 0;
    for s in input.iter() {
        let literal_length = s.len();
        let encoded_length = encode(s).len();
        delta += encoded_length - literal_length;
    }
    delta
}
