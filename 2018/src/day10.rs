use regex::Regex;
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Light> {
    let re = Regex::new(r"position=<\s*(-?\d+), \s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

    let mut lights = vec![];

    for caps in re.captures_iter(input) {
        let velocity = (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap());
        let position = (caps.get(3).unwrap().as_str().parse().unwrap(), caps.get(4).unwrap().as_str().parse().unwrap());

        lights.push(Light::new(position, velocity));
    }

    lights
}

#[aoc(day10, part1)]
pub fn part1(lights: &[Light]) -> String {
    let mut lights = lights.to_vec(); //
    for i in 0..5 {
        println!("{}", get_message(&lights));
        lights
            .iter_mut()
            .for_each(|light| light.calc_new_position());

        // std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{}", i);
    }

    String::new()
}

#[derive(Debug, Clone)]
pub struct Light {
    velocity: (i32, i32),
    position: (i32, i32)
}

impl Light {
    fn new(velocity: (i32, i32), position: (i32, i32)) -> Light {
        Self { 
            velocity,
            position
        }
    }

    fn calc_new_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

fn get_message(lights: &[Light]) -> String {
    let min_x = lights.iter().min_by_key(|l| l.position.1).unwrap().position.1;
    let min_y = lights.iter().min_by_key(|l| l.position.0).unwrap().position.0;
    let max_x = lights.iter().max_by_key(|l| l.position.1).unwrap().position.1;
    let max_y = lights.iter().max_by_key(|l| l.position.0).unwrap().position.0;

    let mut chars: Vec<Vec<char>> = vec![];

    for x in 0..min_x.abs() + max_x.abs() + 1 {
        let mut row = Vec::new();
        for y in 0..min_y.abs() + max_y.abs() + 1 {
            if get_at_position(lights, y, x) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        row.push('\n');
        chars.push(row);
    }

    let chars: Vec<String> = chars.iter()
        .map(|row| row.iter().collect::<String>())
        .collect_vec();

    let mut s = String::new();

    for row in chars.iter() {
        s.push_str(row);
    }

    s
}

fn get_at_position(v: &[Light], x: i32, y: i32) -> bool {
    v.iter()
        .any(|l| l.position.0 == x && l.position.1 == y)
}


#[test]
fn test() {
    let s = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    let s = generator(s);

    assert_eq!(part1(&s), "asdf")
}