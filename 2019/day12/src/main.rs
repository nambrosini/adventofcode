use day12;

fn main() {
    let input: String = std::fs::read_to_string("input").unwrap();

    day12::solve(&input);
}