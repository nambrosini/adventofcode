use std::collections::HashMap;

type Filesystem = HashMap<String, Vec<(usize, String)>>;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> HashMap<String, usize> {
    let mut map: Filesystem = HashMap::new();
    let mut current_folder: Vec<&str> = Vec::new();
    let mut files: Vec<(usize, String)> = Vec::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        if line[0] == "$" {
            if line[1] == "cd" {
                let folder_absolute_path = build_absolute_path(&current_folder, None);
                let entry = map.entry(folder_absolute_path).or_insert_with(Vec::new);
                entry.append(&mut files);
                if line[2] == ".." {
                    current_folder.pop();
                } else {
                    current_folder.push(line[2]);
                }
            }
        } else if line[0] == "dir" {
            let absolute_path = build_absolute_path(&current_folder, Some(line[1]));
            files.push((0, absolute_path));
        } else {
            let absolute_path = build_absolute_path(&current_folder, Some(line[1]));
            files.push((line[0].parse().unwrap(), absolute_path));
        }
    }

    let folder_absolute_path = build_absolute_path(&current_folder, None);
    let entry = map.entry(folder_absolute_path).or_insert_with(Vec::new);
    entry.append(&mut files);

    let mut queue: Vec<String> = vec!["/".to_string()];
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        let content = &map[&current];

        let mut folders: Vec<String> = content
            .iter()
            .filter(|(size, name)| size == &0 && !folder_sizes.keys().any(|f| f == name))
            .map(|(_, name)| name.clone())
            .collect();
        if folders.is_empty() {
            let mut size = 0;
            for (s, n) in content {
                size += if s == &0 { folder_sizes[n] } else { *s }
            }
            folder_sizes.insert(current.to_string(), size);
        } else {
            queue.push(current);
            queue.append(&mut folders);
        }
    }

    folder_sizes
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, usize>) -> usize {
    input
        .iter()
        .filter(|(_, v)| v <= &&100_000)
        .map(|(_, v)| *v)
        .sum()
}

#[aoc(day07, part2)]
pub fn part2(input: &HashMap<String, usize>) -> usize {
    let needed = 30000000 - (70000000 - input["/"]);

    input
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| v >= &needed)
        .min()
        .unwrap()
}

fn build_absolute_path(path: &[&str], filename: Option<&str>) -> String {
    let mut s = String::from("/");
    for p in path.iter().skip(1) {
        s.push_str(p);
        s.push('/');
    }

    if let Some(name) = filename {
        s.push_str(name);
        if !name.contains('.') {
            s.push('/')
        }
    }

    s
}

#[test]
fn test() {
    let s = std::fs::read_to_string("tests/test07.txt").unwrap();

    let got = part1(&generator(&s));

    assert_eq!(got, 95437);
}

#[test]
fn test_part2() {
    let s = std::fs::read_to_string("tests/test07.txt").unwrap();

    let got = part2(&generator(&s));

    assert_eq!(got, 24933642)
}

#[test]
fn test_build_absolute_path_empty() {
    let path = vec![];
    assert_eq!("/", build_absolute_path(&path, None));
}

#[test]
fn test_build_absolute_path_root() {
    let path = vec!["/"];
    assert_eq!("/", build_absolute_path(&path, None))
}

#[test]
fn test_build_absolute_path_dir() {
    let path = vec!["/", "a", "b"];
    assert_eq!("/a/b/c/", build_absolute_path(&path, Some("c")))
}

#[test]
fn test_build_absolute_path_file() {
    let path = vec!["/", "a"];
    assert_eq!("/a/b.tmp", build_absolute_path(&path, Some("b.tmp")));
}
