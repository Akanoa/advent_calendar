use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(PartialEq, Debug)]
enum OpCode {
    ADD,
    MULTIPLY,
    STOP,
    STORE,
    OUTPUT
}

impl OpCode {
    fn get_increment(opcode: OpCode) -> usize {
        match opcode {
            OpCode::ADD | OpCode::MULTIPLY => 4,
            OpCode::STORE | OpCode::OUTPUT => 2,
            OpCode::STOP => 0
        }
    }

    fn from_str(string : String) -> OpCode {

        match &string[..] {
            "01" => OpCode::ADD,
            "02" => OpCode::MULTIPLY,
            "03" => OpCode::STORE,
            "04" => OpCode::OUTPUT,
            "99" => OpCode::STOP,
            _ => panic!("Unknown opcode {}", string)
        }
    }

    fn get_opcode_and_modes_from_str(string: String) -> (Parameter, OpCode) {

        // sanitize input
        let mut input = format!("{:0>4}", string)
            .chars()
            .rev()
            .take(4)
            .collect::<Vec<char>>();

        let opcode_string = (&input[0..2]).to_vec().into_iter().rev().collect::<String>();
        let mode_string = (&input[2..4]).to_vec().into_iter().collect::<String>();

        let param = Parameter::from_str(mode_string);
        let opcode = OpCode::from_str(opcode_string);
        (param, opcode)
    }
}

impl From<u32> for OpCode {
    fn from(x: u32) -> OpCode {
        match x {
            1 => OpCode::ADD,
            2 => OpCode::MULTIPLY,
            3 => OpCode::STORE,
            4 => OpCode::OUTPUT,
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

#[derive(PartialEq, Debug)]
enum Mode {
    Positional,
    Immediate
}

#[derive(PartialEq, Debug)]
struct Parameter {
    first_operand: Mode,
    second_operand: Mode,
}

impl Parameter {
    fn from_str(string: String) -> Parameter {

        if string.len() < 2 {
            panic!("The mode string is too short, must be at least 2 characters, {}", string)
        }


        let mut result = string
            .chars()
            .rev()
            .take(2)
            .map(|c| {
                match c {
                    '0' => Some(Mode::Positional),
                    '1' => Some(Mode::Immediate),
                    _ => None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Mode>>();

        if result.len() < 2 {
            panic!("The mode vec is too short, must be at least 2 digit")
        }


        // I'm sure there is exactly 2 values in the Vec
        let first = result.pop().unwrap();
        let second = result.pop().unwrap();

        Parameter {
            first_operand: first,
            second_operand: second
        }
    }
}

///
/// Loads program
///
///
pub fn read_program_file(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .split(',')
        .map(|x| x.to_string())
        .collect();

    Ok(result)
}

pub fn computer(mut memory :Vec<u32>) -> Vec<u32> {

    let mut instruction_cursor: usize = 0;
    loop {
        let opcode =  OpCode::from(memory[instruction_cursor + Command::OpCode as usize]);

        if opcode == OpCode::STOP {
            break;
        };


        match opcode {
            OpCode::ADD | OpCode::MULTIPLY => {

                let operand_address_1 =  memory[instruction_cursor + Command::OperandAddress1 as usize] as usize;
                let operand_address_2 =  memory[instruction_cursor + Command::OperandAddress2 as usize] as usize;

                let operand_1 = memory[operand_address_1];
                let operand_2 = memory[operand_address_2];

            },
            OpCode::OUTPUT | OpCode::STORE => {

            },
            _ => panic!("Unknown opcode")
        }



//        let operand_address_1 =  memory[instruction_cursor + Command::OperandAddress1 as usize] as usize;
//        let operand_address_2 =  memory[instruction_cursor + Command::OperandAddress2 as usize] as usize;
//
//        let operand_1 = memory[operand_address_1];
//        let operand_2 = memory[operand_address_2];
//
//        let mut result  = None ;
//        match opcode {
//            OpCode::ADD => result = Some(operand_1 + operand_2),
//            OpCode::MULTIPLY => result = Some(operand_1 * operand_2),
//            OpCode::OUTPUT => println!("The content of address #{}"),
//            _ => panic!("Unknown opcode")
//        };
//
//        let result_address =  memory[instruction_cursor + Command::ResultAddress as usize] as usize;
//        memory[result_address] = result;
//
//        instruction_cursor += OpCode::get_increment(opcode);
    }

    memory
}


#[cfg(test)]
mod tests {
    use super::{OpCode, computer, Parameter, Mode, read_program_file};
    use std::path::PathBuf;

    #[test]
    fn test_opcode_to_increment() {
        assert_eq!(OpCode::get_increment(OpCode::ADD), 4);
        assert_eq!(OpCode::get_increment(OpCode::MULTIPLY), 4);
        assert_eq!(OpCode::get_increment(OpCode::STORE), 2);
        assert_eq!(OpCode::get_increment(OpCode::OUTPUT), 2);
        assert_eq!(OpCode::get_increment(OpCode::STOP), 0);
    }

    #[test]
    fn test_integer_to_opcode() {
        assert_eq!(OpCode::from(1), OpCode::ADD);
        assert_eq!(OpCode::from(2), OpCode::MULTIPLY);
        assert_eq!(OpCode::from(3), OpCode::STORE);
        assert_eq!(OpCode::from(4), OpCode::OUTPUT);
        assert_eq!(OpCode::from(99), OpCode::STOP);
    }

    #[test]
    fn test_opcode_from_str() {
        assert_eq!(OpCode::from_str("01".to_string()), OpCode::ADD);
        assert_eq!(OpCode::from_str("02".to_string()), OpCode::MULTIPLY);
        assert_eq!(OpCode::from_str("03".to_string()), OpCode::STORE);
        assert_eq!(OpCode::from_str("04".to_string()), OpCode::OUTPUT);
        assert_eq!(OpCode::from_str("99".to_string()), OpCode::STOP);
    }

    #[test]
    fn test_computer() {
        assert_eq!(computer(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99], "Must be able to add two numbers");
        assert_eq!(computer(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99], "Must be able to multiply two numbers");
        assert_eq!(computer(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801], "Must be able to multiply two numbers and store the result");
        assert_eq!(computer(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99], "Must be able to handle complex program");
    }

    #[test]
    fn test_create_parameters_from_str() {
        assert_eq!(Parameter::from_str("0000".to_string()), Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional });
        assert_eq!(Parameter::from_str("001".to_string()), Parameter { first_operand: Mode::Positional, second_operand: Mode::Immediate });
        assert_eq!(Parameter::from_str("0010".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Positional });
        assert_eq!(Parameter::from_str("11".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Immediate });
    }

    #[test]
    fn test_get_opcode_and_modes_from_str() {
        assert_eq!(OpCode::get_opcode_and_modes_from_str("001".to_string()),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional }, OpCode::ADD));
    }

    #[test]
    fn test_read_program_file() {
        let path = PathBuf::from("./assets/dev_program.txt");
        let results = read_program_file(path).unwrap();
        assert_eq!(results, vec!["0001","0","0","0","99"], "Must read the right value from file")
    }
}

