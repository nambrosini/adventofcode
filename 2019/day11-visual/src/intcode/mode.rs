use std::convert::TryFrom;

pub enum Mode {
    Pos,
    Imm,
    Rel,
}

impl TryFrom<i64> for Mode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Pos),
            1 => Ok(Mode::Imm),
            2 => Ok(Mode::Rel),
            _ => Err(format!("Unknown mode: {}", value)),
        }
    }
}
