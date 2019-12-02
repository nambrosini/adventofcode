use std::process;

pub fn run(numbers: &mut [i32]) -> Result<(), &'static str> {
    let mut counter: usize = 0;

    loop {
        match numbers[counter] {
            1 => add(counter, numbers).unwrap(),
            2 => multiply(counter, numbers).unwrap(),
            99 => {
                println!("HALT!: {}", numbers[0]);
                process::exit(0);
            },
            _ => {
                println!("Opcode not found: {}", numbers[counter]);
                process::exit(1);
            }
        }

        counter += 4;
    }

    Ok(())
}

pub fn add(actual_index: usize, list: &mut [i32]) -> Result<(), &'static str> {
    if actual_index + 3 >= list.len() {
        return Err("Index out of bounds");
    }

    let first_number = list[list[actual_index + 1] as usize];
    let second_number = list[list[actual_index + 2] as usize];
    let result_index = list[actual_index + 3] as usize;

    list[result_index] = first_number + second_number;

    Ok(())
}

pub fn multiply(actual_index: usize, list: &mut [i32]) -> Result<(), &'static str> {
    if actual_index + 3 >= list.len() {
        return Err("Index out of bounds");
    }

    let first_number = list[list[actual_index + 1] as usize];
    let second_number = list[list[actual_index + 2] as usize];
    let result_index = list[actual_index + 3] as usize;

    list[result_index] = first_number * second_number;

    Ok(())
}