use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Orbit {
    u: String,
    v: String,
}


#[derive(Debug)]
enum ParseEdgeError {
    Empty,
}

type OrbitMap = HashMap<String, Vec<Orbit>>;
type Reachables = HashMap<String, HashSet<String>>;
type Dists = HashMap<String, usize>;

impl FromStr for Orbit {
    type Err = ParseEdgeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(')');
        let u = it.next().ok_or(ParseEdgeError::Empty)?.to_owned();
        let v = it.next().ok_or(ParseEdgeError::Empty)?.to_owned();
        Ok(Orbit { u, v })
    }
}

fn calc_dists_from(pos: &str, orbit_map: &OrbitMap, dists: &mut Dists, dist: usize) {
    dists.insert(pos.into(), dist);
    for e in &orbit_map[pos] {
        calc_dists_from(&e.v, orbit_map, dists, dist + 1);
    }
}

fn calc_reachables_from(pos: &str, orbit_map: &OrbitMap, reachables: &mut Reachables) {
    reachables.insert(pos.to_string(), HashSet::new());

    for o in &orbit_map[pos] {
        reachables.get_mut(pos).unwrap().insert(o.v.clone());
        calc_reachables_from(&o.v, orbit_map, reachables);
        let child: Vec<_> = reachables[&o.v].iter().cloned().collect();
        for c in child {
            reachables.get_mut(pos).unwrap().insert(c.clone());
        }
    }
}

pub fn solve_part_1(input: &str) -> (usize, usize) {
    let edges: Vec<Orbit> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut orbit_map = HashMap::new();
    for e in edges {
        let key = &e.u;
        orbit_map.entry(e.v.clone()).or_insert_with(Vec::new);
        let es = orbit_map.entry(key.clone()).or_insert_with(Vec::new);
        es.push(e);
    }

    let mut dists: HashMap<String, usize> = HashMap::new();
    calc_dists_from("COM", &orbit_map, &mut dists, 0);

    let dist: usize = dists.values().sum();

    let mut reachables: HashMap<String, HashSet<String>> = HashMap::new();
    calc_reachables_from("COM", &orbit_map, &mut reachables);

    // Get furthest from "COM" that has both SAN and YOU reachable
    let sanyou = reachables
        .iter()
        .filter(|(_, v)| v.contains("SAN") && v.contains("YOU"))
        .max_by_key(|(k, _)| dists[*k]);

    let hops = if let Some(v) = sanyou {
        let dist_sanyou = dists[v.0];
        let dist_san = dists["SAN"];
        let dist_you = dists["YOU"];
        (dist_san - dist_sanyou) + (dist_you - dist_sanyou) - 2
    } else {
        0
    };

    (dist, hops)
}

pub fn solve_part_2(input: &str) {
    let edges: Vec<Orbit> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut orbit_map = HashMap::new();
    for e in edges {
        let key = &e.u;
        orbit_map.entry(e.v.clone()).or_insert_with(Vec::new);
        let es = orbit_map.entry(key.clone()).or_insert_with(Vec::new);
        es.push(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_one() {
        let input = fs::read_to_string("test.in").unwrap();

        assert_eq!(solve_part_1(&input).0, 42);
    }

    #[test]
    fn test_two() {
        let input = fs::read_to_string("test1.in").unwrap();

        assert_eq!(solve_part_1(&input).1, 4);
    }
}
