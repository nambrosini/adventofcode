pub fn execute_with_fixed_state(x: i32, y: i32, input: &mut [i32]) -> i32 {
    input[1] = x;
    input[2] = y;

    execute_op(input)
}

fn execute_op(input: &mut [i32]) -> i32 {
    let mut counter = 0;

    loop {
        match input[counter] {
            1 => {
                input[input[counter + 3] as usize] =
                    input[input[counter + 1] as usize] + input[input[counter + 2] as usize]
            }
            2 => {
                input[input[counter + 3] as usize] =
                    input[input[counter + 1] as usize] * input[input[counter + 2] as usize]
            }
            99 => return input[0],
            _ => panic!("Unknown op code: {}", input[counter]),
        }

        counter += 4;
    }
}

pub fn find_noun_and_verb(input: &[i32]) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_copy = Vec::from(input);
            if execute_with_fixed_state(noun, verb, &mut input_copy) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Noun and verb not found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut list = vec![1, 0, 0, 0, 99];

        assert_eq!(2, execute_op(&mut list));
        assert_eq!(list, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_two() {
        let mut list = vec![2, 3, 0, 3, 99];

        assert_eq!(2, execute_op(&mut list));
        assert_eq!(list, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_three() {
        let mut list = vec![2, 4, 4, 5, 99, 0];

        assert_eq!(2, execute_op(&mut list));
        assert_eq!(list, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_four() {
        let mut list = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        assert_eq!(30, execute_op(&mut list));
        assert_eq!(list, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
