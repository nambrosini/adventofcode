use std::convert::TryFrom;

pub fn execute_with_fixed_state(x: i32, y: i32, input: &mut [i32]) -> Result<i32, &'static str> {
    input[1] = x;
    input[2] = y;

    execute_op(input)
}


fn execute_op(memory: &mut [i32]) -> Result<i32, &'static str> {
    let mut address = 0;

    loop {
        let op_code = IntCode::try_from(memory[address])?;
        match op_code {
            IntCode::Add => {
                let (p1, p2) = (mem_get(memory, address + 1), mem_get(memory, address + 2));
                mem_set(memory, address + 3, p1 + p2);
            }
            IntCode::Mult => {
                let (p1, p2) = (mem_get(memory, address + 1), mem_get(memory, address + 2));
                mem_set(memory, address + 3, p1 * p2);
            }
            IntCode::Quit => return Ok(memory[0]),
        }

        address += 4;
    }
}

fn mem_get(memory: &[i32], address: usize) -> i32 {
    memory[memory[address] as usize]
}

fn mem_set(memory: &mut [i32], address: usize, v: i32) {
    memory[memory[address] as usize] = v;
}

pub fn find_noun_and_verb(input: &[i32]) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_copy = Vec::from(input);
            if execute_with_fixed_state(noun, verb, &mut input_copy).unwrap() == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Noun and verb not found!");
}

enum IntCode {
    Add,
    Mult,
    Quit,
}

impl TryFrom<i32> for IntCode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mult),
            99 => Ok(Self::Quit),
            _ => Err("Unknown IntCode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut list = vec![1, 0, 0, 0, 99];

        execute_op(&mut list).unwrap();
        assert_eq!(list, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_two() {
        let mut list = vec![2, 3, 0, 3, 99];

        assert_eq!(execute_op(&mut list).unwrap(), 2);
        assert_eq!(list, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_three() {
        let mut list = vec![2, 4, 4, 5, 99, 0];

        assert_eq!(execute_op(&mut list).unwrap(), 2);
        assert_eq!(list, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_four() {
        let mut list = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        assert_eq!(execute_op(&mut list).unwrap(), 30);
        assert_eq!(list, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
