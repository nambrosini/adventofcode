use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct WaitingHall {
    seats: Vec<Vec<Status>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Status::Empty => 'L',
            Status::Occupied => '#',
            Status::Floor => '.',
        };

        write!(f, "{}", res)
    }
}

impl TryFrom<char> for Status {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Status::Occupied,
            '.' => Status::Floor,
            'L' => Status::Empty,
            _ => unreachable!(),
        })
    }
}

impl WaitingHall {
    pub fn new(seats: Vec<Vec<Status>>) -> Self {
        Self { seats }
    }

    pub fn run_part1(&mut self) {
        loop {
            let save = self.seats.clone();
            let occupied = self.compute_occupied_count_part1();

            for (i, row) in occupied.iter().enumerate() {
                for (j, &seat) in row.iter().enumerate() {
                    if self.seats[i][j] == Status::Occupied {
                        if seat >= 4 {
                            self.seats[i][j] = Status::Empty;
                        }
                    } else if self.seats[i][j] == Status::Empty && seat == 0 {
                        self.seats[i][j] = Status::Occupied;
                    }
                }
            }

            if self.seats == save {
                return;
            }
        }
    }

    fn compute_occupied_count_part1(&self) -> Vec<Vec<usize>> {
        let mut occupied: Vec<Vec<usize>> = vec![vec![0; self.seats[0].len()]; self.seats.len()];
        for (i, r) in occupied.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                *e = self.count_occupied_part1(i, j);
            }
        }

        occupied
    }

    pub fn count_occupied_part1(&self, x: usize, y: usize) -> usize {
        let mut occupied = 0;

        let start_x = if x.checked_sub(1) == None { 0 } else { x - 1 };
        let start_y = if y.checked_sub(1) == None { 0 } else { y - 1 };
        let end_x = if x + 1 >= self.seats.len() {
            self.seats.len() - 1
        } else {
            x + 1
        };
        let end_y = if y + 1 >= self.seats[0].len() {
            self.seats[0].len() - 1
        } else {
            y + 1
        };

        for i in start_x..=end_x {
            for j in start_y..=end_y {
                if i == x && j == y {
                    continue;
                }
                if self.seats[i][j] == Status::Occupied {
                    occupied += 1;
                }
            }
        }

        occupied
    }

    pub fn count_occupied_part2(&self, x: usize, y: usize) -> usize {
        let dirs: Vec<(i32, i32)> = vec![
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        let mut occupied = 0;

        for dir in dirs {
            let mut i = x as i32;
            let mut j = y as i32;
            loop {
                i += if i + dir.0 < 0 || i + dir.0 >= self.seats.len() as i32 {
                    break;
                } else {
                    dir.0
                };

                j += if j + dir.1 < 0 || j + dir.1 >= self.seats[i as usize].len() as i32 {
                    break;
                } else {
                    dir.1
                };

                match self.seats[i as usize][j as usize] {
                    Status::Occupied => {
                        occupied += 1;
                        break;
                    }
                    Status::Empty => break,
                    _ => continue,
                }
            }
        }

        occupied
    }

    fn compute_occupied_count_part2(&self) -> Vec<Vec<usize>> {
        let mut occupied: Vec<Vec<usize>> = vec![vec![0; self.seats[0].len()]; self.seats.len()];
        for (i, r) in occupied.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                *e = self.count_occupied_part2(i, j);
            }
        }

        occupied
    }

    pub fn run_part2(&mut self) {
        loop {
            let save = self.seats.clone();
            let occupied = self.compute_occupied_count_part2();

            for (i, row) in occupied.iter().enumerate() {
                for (j, &seat) in row.iter().enumerate() {
                    if self.seats[i][j] == Status::Occupied {
                        if seat >= 5 {
                            self.seats[i][j] = Status::Empty;
                        }
                    } else if self.seats[i][j] == Status::Empty && seat == 0 {
                        self.seats[i][j] = Status::Occupied;
                    }
                }
            }

            if self.seats == save {
                return;
            }
        }
    }
}

impl fmt::Display for WaitingHall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.seats {
            for seat in line {
                write!(f, "{}", seat)?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> WaitingHall {
    WaitingHall::new(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.try_into().unwrap()).collect())
            .collect(),
    )
}

#[aoc(day11, part1)]
pub fn part1(input: &WaitingHall) -> usize {
    let mut input = input.clone();
    input.run_part1();

    input
        .seats
        .iter()
        .flatten()
        .filter(|&s| s.clone() == Status::Occupied)
        .count()
}

#[aoc(day11, part2)]
pub fn part2(input: &WaitingHall) -> usize {
    let mut input = input.clone();
    input.run_part2();

    input
        .seats
        .iter()
        .flatten()
        .filter(|&s| s.clone() == Status::Occupied)
        .count()
}

#[test]
fn sample1_part1() {
    let one = generator("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL");

    assert_eq!(part1(&one), 37);
}

#[test]
fn sample1_part2() {
    let one = generator("L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL");

    assert_eq!(part2(&one), 26);
}
