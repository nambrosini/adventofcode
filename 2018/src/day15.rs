// use std::cmp::Ordering;
// use std::collections::HashMap;
//
// #[aoc_generator(day15)]
// pub fn generator(input: &str) -> Map {
//     input.into()
// }
//
// pub struct Map {
//     points: Vec<Vec<char>>,
//     foes: HashMap<(usize, usize), Foe>
// }
//
// impl Map {
//     fn simulate(&mut self) {
//
//     }
//
//     fn has_nearby_foes(&self, i: usize, j: usize) -> Option<Point<i32>> {
//         let nearby = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
//         let i = i as i32;
//         let j = j as i32;
//
//         for near in &nearby {
//             let xi = (i + near.0) as usize;
//             let yi = (j + near.1) as usize;
//
//             match self.points[xi][yi] {
//                 'E' | 'G' => return Some(Point::new(xi as i32, yi as i32)),
//                 _ => continue
//             }
//         }
//
//         None
//     }
// }
//
// // impl std::fmt::Display for Map {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         Ok(())
// //     }
// // }
//
// impl From<&str> for Map {
//     fn from(s: &str) -> Self {
//         let points: Vec<Vec<char>> = s.lines()
//             .map(|l| l.chars().collect())
//             .collect();
//
//         let mut foes: HashMap<(usize, usize), Foe> = HashMap::new();
//
//         for (i, p) in points.iter().enumerate() {
//             for (j, e) in p.iter().enumerate() {
//                 match e {
//                     'E' => {
//                         foes.insert((i, j), Foe::Elf(200));
//                     },
//                     'G' => {
//                         foes.insert((i, j), Foe::Goblin(200));
//                     },
//                     _ => continue
//                 }
//             }
//         }
//
//         Self {
//             points,
//             foes
//         }
//     }
// }
//
// enum Foe {
//     Elf(i32),
//     Goblin(i32)
// }
//
// impl Foe {
//     fn other(&self) -> Self {
//         match self {
//             Foe::Elf(_) => Foe::Goblin(0),
//             Foe::Goblin(_) => Foe::Elf(0)
//         }
//     }
//
//     fn is_elf(&self) -> bool {
//         match self {
//             Self::Elf(_) => true,
//             Self::Goblin(_) => false
//         }
//     }
// }
//
// impl PartialEq for Foe {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             Foe::Elf(_) => other.is_elf(),
//             Foe::Goblin(_) => !other.is_elf()
//         }
//     }
// }
//
// #[derive(Eq, PartialEq, Clone, Copy)]
// struct Point<T> {
//     x: T,
//     y: T
// }
//
// impl<T> Point<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y
//         }
//     }
// }
//
// impl<T: std::ops::Add<Output=T>> std::ops::Add for Point<T> {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y
//         }
//     }
// }
//
// impl<T: std::ops::AddAssign> std::ops::AddAssign for Point<T> {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//     }
// }
//
// impl std::cmp::Ord for Point<i32> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.x.cmp(&other.x) {
//             Ordering::Less => Ordering::Less,
//             Ordering::Equal => self.y.cmp(&other.y),
//             Ordering::Greater => Ordering::Greater
//         }
//     }
// }
//
// impl std::cmp::PartialOrd for Point<i32> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
