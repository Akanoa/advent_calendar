use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

#[derive(PartialEq)]
enum OpCode {
    ADD,
    MULTIPLY,
    STOP
}

impl From<u32> for OpCode {
    fn from(x: u32) -> OpCode {
        match x {
            1 => OpCode::ADD,
            2 => OpCode::MULTIPLY,
            99 => OpCode::STOP,
            _ => panic!("Unknown opcode {}", x)
        }
    }
}


enum Command {
    OpCode = 0,
    OperandAddress1,
    OperandAddress2,
    ResultAddress,
}

///
/// Loads program
///
///
fn read_program_file(path: PathBuf) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .split(',')
        .map(|x| {
            match x.parse::<u32>() {
                Ok(x) => Some(x),
                Err(_e) => None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();


    Ok(result)
}

fn computer(mut memory :Vec<u32>) -> Vec<u32> {

    let mut instruction_cursor: usize = 0;
    loop {
        let opcode =  OpCode::from(memory[instruction_cursor + Command::OpCode as usize]);

        if opcode == OpCode::STOP {
            break;
        };

        let operand_address_1 =  memory[instruction_cursor + Command::OperandAddress1 as usize] as usize;
        let operand_address_2 =  memory[instruction_cursor + Command::OperandAddress2 as usize] as usize;

        let operand_1 = memory[operand_address_1];
        let operand_2 = memory[operand_address_2];

        let result = match opcode {
            OpCode::ADD => operand_1 + operand_2,
            OpCode::MULTIPLY => operand_1 * operand_2,
            _ => panic!("Unknown opcode")
        };

        let result_address =  memory[instruction_cursor + Command::ResultAddress as usize] as usize;
        memory[result_address] = result;

        instruction_cursor += 4;
    }

    memory
}

pub fn part_1(input_1: u32, input_2: u32) -> Vec<u32> {
    let path = PathBuf::from("./assets/program.txt");
    let mut memory = read_program_file(path).unwrap();
    // Define inputs
    memory[1] = input_1;
    memory[2] = input_2;
    // Compute
    computer(memory)
}

#[cfg(test)]
mod tests {
    use crate::{computer, read_program_file};
    use std::path::PathBuf;

    #[test]
    fn test_computer() {
        assert_eq!(computer(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99], "Must be able to add two numbers");
        assert_eq!(computer(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99], "Must be able to multiply two numbers");
        assert_eq!(computer(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801], "Must be able to multiply two numbers and store the result");
        assert_eq!(computer(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99], "Must be able to handle complex program");
    }

    #[test]
    fn test_read_program_file() {
        let path = PathBuf::from("./assets/dev_program.txt");
        let results = read_program_file(path).unwrap();
        assert_eq!(results, vec![1,0,0,0,99], "Must read the right value from file")
    }
}