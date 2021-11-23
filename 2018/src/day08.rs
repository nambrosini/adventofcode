use itertools::Itertools;

#[aoc_generator(day08)]
pub fn generator(input: &str) -> Node {
    let mut input: Vec<usize> = input.split(' ').map(|s| s.parse().unwrap()).collect_vec();

    let header = (input.remove(0), input.remove(0));
    let mut children = vec![];

    for _ in 0..header.0 {
        children.push(get_node(&mut input));
    }

    let metadata = input.clone();

    Node::new(header, children, metadata)
}

#[aoc(day08, part1)]
pub fn part1(input: &Node) -> usize {
    count_metadata(input)
}

#[aoc(day08, part2)]
pub fn part2(input: &Node) -> usize {
    get_value(input)
}

fn count_metadata(node: &Node) -> usize {
    let mut metadata = node.metadata.iter().sum();

    for n in &node.children {
        metadata += count_metadata(n);
    }

    metadata
}

fn get_node(input: &mut Vec<usize>) -> Node {
    let header = (input.remove(0), input.remove(0));
    let mut children = vec![];

    for _ in 0..header.0 {
        children.push(get_node(input));
    }

    let mut metadata = vec![];

    for _ in 0..header.1 {
        metadata.push(input.remove(0));
    }

    Node::new(header, children, metadata)
}

fn get_value(node: &Node) -> usize {
    if node.header.0 == 0 {
        return node.metadata.iter().sum();
    }

    let mut sum = 0;

    for i in &node.metadata {
        if *i > node.children.len() {
            continue;
        }
        sum += get_value(&node.children[*i - 1]);
    }

    sum
}

pub struct Node {
    header: (usize, usize),
    children: Vec<Node>,
    metadata: Vec<usize>
}

impl Node {
    fn new(header: (usize, usize), children: Vec<Node>, metadata: Vec<usize>) -> Node {
        Self {
            header,
            children,
            metadata,
        }
    }
}

#[test]
fn test() {
    let s = generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");

    assert_eq!(part1(&s), 138);
}


#[test]
fn test2() {
    let s = generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");

    assert_eq!(part2(&s), 66);
}