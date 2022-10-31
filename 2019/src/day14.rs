use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn generator(input: &str) ->  HashMap<String, Reaction> {
    input.lines()
        .map(|l| {
            let r: Reaction = l.into();
            (r.output.0.clone(), r)
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(rules: &HashMap<String, Reaction>) -> usize {
    let mut resources = HashMap::new();
    produce("FUEL", 1, rules, &mut resources)
}

#[aoc(day14, part2)]
pub fn part2(rules: &HashMap<String, Reaction>) -> usize {
    let mut resources = HashMap::new();
    let p1 = produce("FUEL", 1, rules, &mut resources);
    let tgt: usize = 1_000_000_000_000;
    let mut begin = 0;
    let mut end = 2 * (tgt / p1);

    loop {
        let mid = (end + begin) / 2;
        let ore = produce("FUEL", mid, rules, &mut HashMap::new());
        let ore_n = produce("FUEL", mid + 1, rules, &mut HashMap::new());

        if ore < tgt && ore_n > tgt {
            return mid;
        } else if ore_n > tgt {
            end = mid - 1;
        } else if ore_n < tgt {
            begin = mid + 1;
        } else {
            unreachable!();
        }
    }
}

pub struct Reaction {
    input: HashMap<String, usize>,
    output: (String, usize)
}

impl From<&str> for Reaction {
    fn from(s: &str) -> Self {
        let mut iter = s.split("=>");
        let ing = iter.next().unwrap();
        let ing: String = ing.replace(',', " ");
        let out = iter.next().unwrap();

        let mut input = HashMap::new();
        let mut it = ing.split_whitespace();
        while let Some(s) = it.next() {
            let cnt = s.parse().unwrap();
            let name = it.next().unwrap();
            input.insert(name.into(), cnt);
        }

        let mut it = out.split_whitespace();
        let cnt = it.next().unwrap().parse().unwrap();
        let name = it.next().unwrap();

        Reaction {
            input,
            output: (name.into(), cnt),
        }
    }
}

fn produce(
    name: &str,
    amount: usize,
    rules: &HashMap<String, Reaction>,
    resources: &mut HashMap<String, usize>,
) -> usize {
    let r = &rules[name];
    let mut ore_cnt = 0;

    let mut produced = 0;

    if let Some(&o) = r.input.get("ORE") {
        let times = (amount + r.output.1 - 1) / r.output.1;
        ore_cnt += o * times;
        produced += r.output.1 * times;
    } else {
        let times = (amount + r.output.1 - 1) / r.output.1;
        for (n, &c) in &r.input {
            let need = c * times;
            let have = *resources.get(n).unwrap_or(&0);
            if have < need {
                ore_cnt += produce(n, need - have, rules, resources);
            }

            // consume the available resources once they're ready
            let r = resources.get_mut(n).unwrap();
            *r -= need;
        }
        produced += r.output.1 * times;
    }

    let r = resources.entry(name.into()).or_insert(0);
    *r += produced;
    ore_cnt
}