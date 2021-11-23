use std::convert::{TryFrom, TryInto};

use super::mode::Mode;

pub enum OpCode {
    Add(Mode, Mode, Mode),
    Mult(Mode, Mode, Mode),
    Save(Mode),
    Out(Mode),
    Quit,
    Jit(Mode, Mode),
    Jif(Mode, Mode),
    Lt(Mode, Mode, Mode),
    Arb(Mode),
    Eq(Mode, Mode, Mode),
}

impl TryFrom<i64> for OpCode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let de = value % 100;
        let c = value / 100 % 10;
        let b = value / 1000 % 10;
        let a = value / 10000 % 10;

        match de {
            1 => Ok(OpCode::Add(c.try_into()?, b.try_into()?, a.try_into()?)),
            2 => Ok(OpCode::Mult(c.try_into()?, b.try_into()?, a.try_into()?)),
            3 => Ok(OpCode::Save(c.try_into()?)),
            4 => Ok(OpCode::Out(c.try_into()?)),
            5 => Ok(OpCode::Jit(c.try_into()?, b.try_into()?)),
            6 => Ok(OpCode::Jif(c.try_into()?, b.try_into()?)),
            7 => Ok(OpCode::Lt(c.try_into()?, b.try_into()?, a.try_into()?)),
            8 => Ok(OpCode::Eq(c.try_into()?, b.try_into()?, a.try_into()?)),
            9 => Ok(OpCode::Arb(c.try_into()?)),
            99 => Ok(OpCode::Quit),
            _ => Err(format!("OpCode not recognized: {}", de)),
        }
    }
}
