
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn turn(direction: &Self, dir: i64) -> Self{
        match direction {
            Direction::Up => {
                if dir == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            },
            Direction::Left => {
                if dir == 0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            },
            Direction::Down => {
                if dir == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            },
            Direction::Right => {
                if dir == 0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }

    pub fn get_direction_param(direction: &Self) -> (i64, i64) {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}