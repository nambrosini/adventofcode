#[aoc_generator(day25)]
fn generator(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}
#[aoc(day25, part1)]
fn part1((door_public, key_public): &(usize, usize)) -> usize {
    let mut door_loop = 0;
    let mut door_value = 1;

    let mut key_loop = 0;
    let mut key_value = 1;

    let mut counter = 0;

    while key_loop == 0 || door_loop == 0 {
        counter += 1;

        door_value = (door_value * 7) % 20201227;
        key_value = (key_value * 7) % 20201227;

        if door_value == *door_public {
            door_loop = counter;
        }

        if key_value == *key_public {
            key_loop = counter;
        }
    }

    let mut door_private = 1;

    for _ in 0..key_loop {
        door_private = (door_private * door_value) % 20201227;
    }

    door_private
}

#[test]
fn sample1() {
    let s = generator("17807724\n5764801");

    assert_eq!(part1(&s), 14897079);
}
