use itertools::Itertools;

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Vec<Component> {
    input.lines().map(|line| line.into()).collect_vec()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Component]) -> usize {
    let starts = input
        .iter()
        .filter(|c| c.input == 0 || c.output == 0)
        .collect_vec();

    let mut max = usize::MIN;

    for start in starts {
        let components = input.iter().filter(|&c| c != start).copied().collect_vec();
        let v = search(&components, start, Port::In, start.input + start.output);

        if v > max {
            max = v;
        }
    }

    max
}

#[aoc(day24, part2)]
pub fn part2(input: &[Component]) -> usize {
    let starts = input
        .iter()
        .filter(|c| c.input == 0 || c.output == 0)
        .collect_vec();

    let mut max = (usize::MIN, usize::MIN);

    for start in starts {
        let components = input.iter().filter(|&c| c != start).copied().collect_vec();
        let v = search_length(&components, start, Port::In, start.input + start.output, 1);

        if v.1 > max.1 {
            max = v;
        }
    }

    max.0
}

fn search(components: &[Component], start: &Component, used: Port, strength: usize) -> usize {
    let compatibles = components
        .iter()
        .filter(|c| match used {
            Port::In => start.output == c.input || start.output == c.output,
            Port::Out => start.input == c.input || start.input == c.output,
        })
        .collect_vec();

    if compatibles.is_empty() {
        return strength;
    }

    let mut max = usize::MIN;

    for comp in compatibles {
        let comps = components
            .iter()
            .filter(|&c| c != comp)
            .cloned()
            .collect_vec();

        let new_strength = strength + comp.input + comp.output;

        let port = if comp.input == start.input || comp.input == start.output {
            Port::In
        } else {
            Port::Out
        };

        let s = search(&comps, comp, port, new_strength);

        if s > max {
            max = s;
        }
    }

    max
}

fn search_length(components: &[Component], start: &Component, used: Port, strength: usize, length: usize) -> (usize, usize) {
    let compatibles = components
        .iter()
        .filter(|c| match used {
            Port::In => start.output == c.input || start.output == c.output,
            Port::Out => start.input == c.input || start.input == c.output,
        })
        .collect_vec();

    if compatibles.is_empty() {
        return (strength, length);
    }

    let mut max = (usize::MIN, usize::MIN);

    for comp in compatibles {
        let comps = components
            .iter()
            .filter(|&c| c != comp)
            .cloned()
            .collect_vec();

        let port = if comp.input == start.input || comp.input == start.output {
            Port::In
        } else {
            Port::Out
        };

        let new_strength = strength + comp.input + comp.output;

        let s = search_length(&comps, comp, port, new_strength, length + 1);

        if s.1 > max.1 {
            max = s;
        }
    }

    max
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Component {
    input: usize,
    output: usize,
}

impl From<&str> for Component {
    fn from(s: &str) -> Self {
        let s = s.split('/').collect_vec();
        Component {
            input: s[0].parse().unwrap(),
            output: s[1].parse().unwrap(),
        }
    }
}

enum Port {
    In,
    Out,
}

#[test]
fn test() {
    let s = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    assert_eq!(part1(&generator(s)), 31);
}
