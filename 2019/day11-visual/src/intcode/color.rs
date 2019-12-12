use std::convert::From;
use std::convert::TryFrom;

#[derive(Clone)]
pub enum Color {
    Black,
    White,
}

impl From<Color> for i64 {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<i64> for Color {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(format!("Color not recognized: {}", value)),
        }
    }
}
