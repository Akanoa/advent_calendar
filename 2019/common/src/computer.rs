use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{VecDeque, HashMap};

#[macro_use]
mod macros {

    macro_rules! get_operand {
        ($memory:expr, $memory_address:expr, $instruction_cursor:expr, $parameter_mode:expr, $text_error:expr, $base:expr) => {

            match $parameter_mode {
                Mode::Immediate => {
                    *$memory.entry($instruction_cursor + $memory_address as i64).or_insert(0)
                },
                Mode::Positional => {
                    let operand_address= match $memory.get(&($instruction_cursor + $memory_address as i64)) {
                        Some(&x) => x,
                        None => panic!("{} (get memory address -- positional): This memory address doesn't exist", $text_error)
                    };

                    *$memory.entry(operand_address).or_insert(0)
                },
                Mode::Relative => {

                let operand_address= match $memory.get(&($instruction_cursor + $memory_address as i64)) {
                    Some(&x) => x + $base as i64,
                    None => panic!("{} (get memory address -- relative): This memory address doesn't exist", $text_error)
                };

                    *$memory.entry(operand_address).or_insert(0)
                }
            }
        };
    }

    macro_rules! get_address {
        ($memory:expr, $offset:expr, $instruction_cursor:expr, $parameter_mode:expr, $base:expr) => {
            {
                let address = *$memory.entry($instruction_cursor + $offset as i64).or_insert(-1);
                match $parameter_mode {
                    Mode::Immediate | Mode::Positional => {
                        address
                    },
                    Mode::Relative => {
                        address + $base
                    }
                }
            }
        };
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum OpCode {
    Add,
    Multiply,
    Stop,
    Store,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Base
}

impl OpCode {
    fn get_increment(opcode: OpCode) -> i64 {
        match opcode {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 4,
            OpCode::Store | OpCode::Output | OpCode::Base => 2,
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => 3,
            OpCode::Stop => 0
        }
    }

    fn from_str(string : String) -> OpCode {

        match &string[..] {
            "01" => OpCode::Add,
            "02" => OpCode::Multiply,
            "03" => OpCode::Store,
            "04" => OpCode::Output,
            "05" => OpCode::JumpIfTrue,
            "06" => OpCode::JumpIfFalse,
            "07" => OpCode::LessThan,
            "08" => OpCode::Equals,
            "09" => OpCode::Base,
            "99" => OpCode::Stop,
            _ => panic!("Unknown opcode {}", string)
        }
    }

    fn get_opcode_and_modes_from_str(string: i64) -> (Parameter, OpCode) {

        // sanitize input
        let input = format!("{:0>5}", string)
            .chars()
            .rev()
            .take(5)
            .collect::<Vec<char>>();

        let opcode_string = (&input[0..2]).to_vec().into_iter().rev().collect::<String>();
        let mode_string = (&input[2..5]).to_vec().into_iter().rev().collect::<String>();

        let param = Parameter::from_str(mode_string);
        let opcode = OpCode::from_str(opcode_string);
        (param, opcode)
    }
}

impl From<u32> for OpCode {
    fn from(x: u32) -> OpCode {
        match x {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Store,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::Base,
            99 => OpCode::Stop,
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
    Immediate,
    Relative
}

#[derive(PartialEq, Debug)]
struct Parameter {
    first_operand: Mode,
    second_operand: Mode,
    result_address: Mode
}

impl Parameter {
    fn from_str(string: String) -> Parameter {

        if string.len() < 3 {
            panic!("The mode string is too short, must be at least 3 characters, {}", string)
        }


        let mut result = string
            .chars()
            .rev()
            .take(3)
            .map(|c| {
                match c {
                    '0' => Some(Mode::Positional),
                    '1' => Some(Mode::Immediate),
                    '2' => Some(Mode::Relative),
                    _ => None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Mode>>();

        if result.len() < 3 {
            panic!("The mode vec is too short, must be at least 3 digit")
        }


        // I'm sure there is exactly 3 values in the Vec
        let result_address = result.pop().unwrap();
        let second = result.pop().unwrap();
        let first = result.pop().unwrap();


        Parameter {
            first_operand: first,
            second_operand: second,
            result_address
        }
    }
}

// Pause at output the return memory
#[derive(Debug, Clone, Copy)]
pub enum ResumeMode {
    Enable,
    Disable
}

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Started,
    Paused,
    Stopped
}

#[derive(Debug)]
pub struct Computer {
    memory: HashMap<i64,i64>,
    output_buffer: Vec<i64>,
    input_data: Option<VecDeque<i64>>,
    pub resume_mode: ResumeMode,
    pub state: State,
    instruction_cursor: i64,
    base: i64
}

impl Computer {
    pub fn new (program: Vec<i64>) -> Computer {

        // Create a memory map from the program vec
        let mut memory = HashMap::new();
        for (key, value) in program.into_iter().enumerate() {
            memory.insert(key as i64, value);
        }

        Computer {
            memory,
            output_buffer: vec![],
            input_data: None,
            resume_mode: ResumeMode::Disable,
            state: State::Started,
            instruction_cursor: 0,
            base: 0
        }
    }

    pub fn set_resume_mode(&mut self, mode: ResumeMode) {
        self.resume_mode = mode;
    }

    pub fn is_setup(&self) -> bool {
        match self.state {
            State::Started => false,
            State::Paused | State::Stopped => true,
        }
    }

    pub fn add_input(&mut self, input : i64) {
        match &mut self.input_data {
            None => {
                self.input_data = Some(VecDeque::new());
                self.input_data.as_mut().unwrap().push_back(input);
            },
            Some(input_data) => {
                input_data.push_back(input);
            }
        }
    }

    pub fn run(&mut self) -> (HashMap<i64,i64>, Vec<i64>) {

        self.state = State::Started;

        loop {

            let increment;

            let opcode_raw =  match self.memory.get(&(self.instruction_cursor + Command::OpCode as i64)) {
                Some(x) => x,
                None => panic!("OPCODE: This memory address doesn't exist")
            };

            let (parameters_mode, opcode) = OpCode::get_opcode_and_modes_from_str(opcode_raw.clone());

            if opcode == OpCode::Stop {
                self.state = State::Stopped;
                self.instruction_cursor += 1;
                break;
            };


            match opcode {
                OpCode::Add | OpCode::Multiply => {

                    let operand_1  = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "OPERAND1", self.base);
                    let operand_2  = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "OPERAND2", self.base);

                    let result = match opcode {
                        OpCode::Add => {
                            operand_1 + operand_2
                        },
                        OpCode::Multiply => operand_1 * operand_2,
                        _ => panic!("Unknown opcode")
                    };

                    let store_address = get_address!(self.memory, Command::ResultAddress, self.instruction_cursor, parameters_mode.result_address, self.base);
                    self.memory.insert(store_address, result);
                    increment = OpCode::get_increment(opcode);

                },
                OpCode::Output | OpCode::Store | OpCode::Base => {

                    let address : i64 = get_address!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, self.base);
                    increment = OpCode::get_increment(opcode);
                    match opcode {
                        OpCode::Store => {
                            match &mut self.input_data {
                                Some(x) => {
                                    match &mut x.pop_front() {
                                        Some(data) => {
                                            *self.memory.entry(address).or_insert(0) = *data;
                                        },
                                        None => panic!("Unable to get value from input Vec")
                                    }
                                },
                                None => panic!("Unable get value to store")
                            };
                        },
                        OpCode::Output => {
                            let value : i64 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "ADRRESS", self.base);
                            self.output_buffer.push(value);

                            match self.resume_mode {
                                ResumeMode::Enable => {
                                    self.state = State::Paused;
                                },
                                ResumeMode::Disable => {},
                            }
                        },
                        OpCode::Base => {
                            let address : i64 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "Base address", self.base);
                            self.base += address;
                        }
                        _ => panic!("Unknown opcode")
                    }

                },
                OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                    let value_checked : i64 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "VALUE CHECKED", self.base);
                    let next_cursor_address : i64 = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "NEXT CURSOR ADDRESS", self.base);
                    let condition_valid: bool = match opcode {
                        OpCode::JumpIfTrue => {
                            if value_checked != 0 {
                                true
                            } else {
                                false
                            }
                        },
                        OpCode::JumpIfFalse => {
                            if value_checked == 0 {
                                true
                            } else {
                                false
                            }
                        },
                        _ => panic!("Unknown opcode")
                    };

                    if condition_valid {

                        self.instruction_cursor = next_cursor_address;
                        increment = 0;
                    } else {
                        increment = OpCode::get_increment(opcode);
                    }
                },
                OpCode::Equals |OpCode::LessThan => {

                    let operand_1 : i64 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "OPERAND 1", self.base);
                    let operand_2 : i64 = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "OPERAND 2", self.base);
                    let result_address : i64 = get_address!(self.memory, Command::ResultAddress, self.instruction_cursor, parameters_mode.result_address, self.base);

                    let result = match opcode {
                        OpCode::Equals => {
                            if operand_1 == operand_2 {
                                1
                            } else {
                                0
                            }
                        },
                        OpCode::LessThan => {
                            if operand_1 < operand_2 {
                                1
                            } else {
                                0
                            }
                        },
                        _ => panic!("Unknown opcode")
                    };

                    *self.memory.entry(result_address).or_insert(0) = result;
                    increment = OpCode::get_increment(opcode);
                }
                _ => {
                    println!("{:?}", opcode);
                    panic!("Unknown opcode")
                }
            }

            self.instruction_cursor += increment;


            match self.state {
                State::Paused => break,
                _ => ()
            }
        }

        (self.memory.clone(), self.output_buffer.clone())
    }
}

///
/// Loads program
///
///
pub fn read_program_file(path: PathBuf) -> Result<Vec<i64>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .split(',')
        .map(|x| match x.parse::<i64>() {
            Ok(value) => Some(value),
            Err(_err) => None
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    Ok(result)
}

pub fn computer(memory :Vec<i64>, input: Option<VecDeque<i64>>) -> (HashMap<i64,i64>, Vec<i64>) {

    let mut computer = Computer::new(memory);
    computer.input_data = input;

    computer.run()
}

pub fn computer_feedback(memory :Vec<i64>, input: Option<VecDeque<i64>>) -> (HashMap<i64,i64>, Vec<i64>) {

    let mut computer = Computer::new(memory);
    computer.resume_mode = ResumeMode::Enable;
    computer.input_data = input;

    computer.run()
}


#[cfg(test)]
mod tests {
    use super::{OpCode, computer, Parameter, Mode, read_program_file};
    use std::path::PathBuf;
    use std::collections::VecDeque;
    use crate::computer::{Computer, ResumeMode, State};
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[macro_use]
    mod macros {

        macro_rules! hashed_map_fill {
            ($typeKey:ty, $typeValue:ty ,$vec:expr) => {
                {
                    let mut hashmap : HashMap<$typeKey, $typeValue> = HashMap::new();
                    for (key, value) in $vec.into_iter().enumerate() {
                        hashmap.insert(key as $typeKey, value);
                    }
                    hashmap
                }

            };
        }
    }


    #[test]
    fn test_opcode_to_increment() {
        assert_eq!(OpCode::get_increment(OpCode::Add), 4);
        assert_eq!(OpCode::get_increment(OpCode::Multiply), 4);
        assert_eq!(OpCode::get_increment(OpCode::Store), 2);
        assert_eq!(OpCode::get_increment(OpCode::Output), 2);
        assert_eq!(OpCode::get_increment(OpCode::JumpIfTrue), 3);
        assert_eq!(OpCode::get_increment(OpCode::JumpIfFalse), 3);
        assert_eq!(OpCode::get_increment(OpCode::LessThan), 4);
        assert_eq!(OpCode::get_increment(OpCode::Equals), 4);
        assert_eq!(OpCode::get_increment(OpCode::Stop), 0);
    }

    #[test]
    fn test_integer_to_opcode() {
        assert_eq!(OpCode::from(1), OpCode::Add);
        assert_eq!(OpCode::from(2), OpCode::Multiply);
        assert_eq!(OpCode::from(3), OpCode::Store);
        assert_eq!(OpCode::from(4), OpCode::Output);
        assert_eq!(OpCode::from(5), OpCode::JumpIfTrue);
        assert_eq!(OpCode::from(6), OpCode::JumpIfFalse);
        assert_eq!(OpCode::from(7), OpCode::LessThan);
        assert_eq!(OpCode::from(8), OpCode::Equals);
        assert_eq!(OpCode::from(99), OpCode::Stop);
    }

    #[test]
    fn test_opcode_from_str() {
        assert_eq!(OpCode::from_str("01".to_string()), OpCode::Add);
        assert_eq!(OpCode::from_str("02".to_string()), OpCode::Multiply);
        assert_eq!(OpCode::from_str("03".to_string()), OpCode::Store);
        assert_eq!(OpCode::from_str("04".to_string()), OpCode::Output);
        assert_eq!(OpCode::from_str("05".to_string()), OpCode::JumpIfTrue);
        assert_eq!(OpCode::from_str("06".to_string()), OpCode::JumpIfFalse);
        assert_eq!(OpCode::from_str("07".to_string()), OpCode::LessThan);
        assert_eq!(OpCode::from_str("08".to_string()), OpCode::Equals);
        assert_eq!(OpCode::from_str("99".to_string()), OpCode::Stop);
    }

    #[test]
    fn test_computer() {

        let empty: Vec<i64> = Vec::new();

        assert_eq!(computer( vec![1, 0, 0, 0, 99], None),
                   (hashed_map_fill!(i64, i64, vec![2, 0, 0, 0, 99]), empty.clone()), "Must be able to add two numbers");
        assert_eq!(computer(vec![2, 3, 0, 3, 99], None),
                   (hashed_map_fill!(i64, i64, vec![2, 3, 0, 6, 99]), empty.clone()), "Must be able to multiply two numbers");
        assert_eq!(computer(vec![2, 4, 4, 5, 99, 0], None),
                   (hashed_map_fill!(i64, i64, vec![2, 4, 4, 5, 99, 9801]), empty.clone()), "Must be able to multiply two numbers and store the result");
        assert_eq!(computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None),
                   (hashed_map_fill!(i64, i64, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]), empty.clone()), "Must be able to handle complex program");
        assert_eq!(computer(vec![1101, 100, -1, 4, 0], None),
                   (hashed_map_fill!(i64, i64, vec![1101, 100, -1, 4, 99]), empty.clone()), "Can handle operation immediate value");
        assert_eq!(computer(vec![3, 0, 4, 0, 99], Some(VecDeque::from(vec![-42]))),
                   (hashed_map_fill!(i64, i64, vec![-42, 0, 4, 0, 99]), vec![-42]), "Able to write in output buffer");

        // --- Positional mode
        // equality
        assert_eq!(computer(vec![3,9,8,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![8]))),
                   (hashed_map_fill!(i64, i64, vec![3,9,8,9,10,9,4,9,99,1,8]), vec![1]), "Able to deal with equality (position mode)");
        assert_eq!(computer(vec![3,9,8,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![12]))),
                   (hashed_map_fill!(i64, i64, vec![3,9,8,9,10,9,4,9,99,0,8]), vec![0]), "Able to deal with non equality (position mode)");
        // less than
        assert_eq!(computer(vec![3,9,7,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![5]))),
                   (hashed_map_fill!(i64, i64, vec![3,9,7,9,10,9,4,9,99,1,8]), vec![1]), "Able to deal with less than (position mode)");
        assert_eq!(computer(vec![3,9,7,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![12]))),
                   (hashed_map_fill!(i64, i64, vec![3,9,7,9,10,9,4,9,99,0,8]), vec![0]), "Able to deal with greater than (position mode)");

        // --- Immediate mode
        // equality
        assert_eq!(computer(vec![3,3,1108,-1,8,3,4,3,99], Some(VecDeque::from(vec![8]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1108,1,8,3,4,3,99]), vec![1]), "Able to deal with equality (immediate mode)");
        assert_eq!(computer(vec![3,3,1108,-1,8,3,4,3,99], Some(VecDeque::from(vec![12]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1108,0,8,3,4,3,99]), vec![0]), "Able to deal with non equality (immediate mode)");
        // less than
        assert_eq!(computer(vec![3,3,1107,-1,8,3,4,3,99], Some(VecDeque::from(vec![5]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1107,1,8,3,4,3,99]), vec![1]), "Able to deal with less than (immediate mode)");
        assert_eq!(computer(vec![3,3,1107,-1,8,3,4,3,99], Some(VecDeque::from(vec![12]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1107,0,8,3,4,3,99]), vec![0]), "Able to deal with greater than (immediate mode)");

        // --- Positional mode
        assert_eq!(computer(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], Some(VecDeque::from(vec![1]))),
                   (hashed_map_fill!(i64, i64, vec![3,12,6,12,15,1,13,14,13,4,13,99,1,1,1,9]), vec![1]), "Should jump if input 1 (position mode)");
        assert_eq!(computer(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], Some(VecDeque::from(vec![0]))),
                   (hashed_map_fill!(i64, i64, vec![3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9]), vec![0]), "Should jump if input 0 (position mode)");

        // --- Immediate mode
        assert_eq!(computer(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Some(VecDeque::from(vec![1]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1105,1,9,1101,0,0,12,4,12,99,1]), vec![1]), "Should jump if input 1 (immediate mode)");
        // --- Positional mode
        assert_eq!(computer(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Some(VecDeque::from(vec![0]))),
                   (hashed_map_fill!(i64, i64, vec![3,3,1105,0,9,1101,0,0,12,4,12,99,0]), vec![0]), "Should jump if input 0 (immediate mode)");

        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                           1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                           999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];


        // equal 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![8])));
        let expected : Vec<i64> = vec![1000];
        assert_eq!(buffer, expected);

        // greater than 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![220])));
        let expected : Vec<i64> = vec![1001];
        assert_eq!(buffer, expected);

        // less than 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![7])));
        let expected : Vec<i64> = vec![999];
        assert_eq!(buffer, expected);

        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let (_, buffer) = computer(program, None);
        let result = buffer.last().unwrap().to_string().chars().into_iter().count();
        assert_eq!(result, 16);


    }

    #[test]
    fn test_create_parameters_from_str() {
        assert_eq!(Parameter::from_str("0000".to_string()), Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional, result_address: Mode::Positional });
        assert_eq!(Parameter::from_str("001".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Positional, result_address: Mode::Positional });
        assert_eq!(Parameter::from_str("0010".to_string()), Parameter { first_operand: Mode::Positional, second_operand: Mode::Immediate, result_address: Mode::Positional });
        assert_eq!(Parameter::from_str("011".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Immediate, result_address: Mode::Positional });
        assert_eq!(Parameter::from_str("111".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Immediate, result_address: Mode::Immediate });
        assert_eq!(Parameter::from_str("211".to_string()), Parameter { first_operand: Mode::Immediate, second_operand: Mode::Immediate, result_address: Mode::Relative });
    }

    #[test]
    fn test_get_opcode_and_modes_from_str() {
        assert_eq!(OpCode::get_opcode_and_modes_from_str(1),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional, result_address: Mode::Positional }, OpCode::Add));
        assert_eq!(OpCode::get_opcode_and_modes_from_str(1002),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Immediate, result_address: Mode::Positional }, OpCode::Multiply));
        assert_eq!(OpCode::get_opcode_and_modes_from_str(99),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional, result_address: Mode::Positional }, OpCode::Stop));
        assert_eq!(OpCode::get_opcode_and_modes_from_str(21102),
                   (Parameter { first_operand: Mode::Immediate, second_operand: Mode::Immediate, result_address: Mode::Relative }, OpCode::Multiply));
    }

    #[test]
    fn test_read_program_file() {
        let path = PathBuf::from("./assets/dev_program.txt");
        let results = read_program_file(path).unwrap();
        assert_eq!(results, vec![1, 0, 0, -42 , 99], "Must read the right value from file")
    }

    #[test]
    fn test_computer_can_store_more_than_one_input() {
        let program = vec![3,5,3,6,99,-1,-1];
        let (memory, _) = computer(program.clone(), Some(VecDeque::from(vec![220, -42])));
        assert_eq!(hashed_map_fill!(i64, i64, vec![3,5,3,6,99,220,-42]), memory);

    }


    #[test]
    fn test_resumable_computer() {

        let program = vec![104, -42,104,48, 99];
        let mut computer = Computer::new(program.clone());
        computer.set_resume_mode(ResumeMode::Enable);
        let (memory, buffer) = computer.run();
        assert_eq!(hashed_map_fill!(i64, i64, vec![104,-42,104,48, 99]), memory);
        assert_eq!(&-42, buffer.last().unwrap());
        assert_eq!(State::Paused, computer.state);
        assert_eq!(2, computer.instruction_cursor);


        let (memory, buffer) = computer.run();
        assert_eq!(hashed_map_fill!(i64, i64, vec![104, -42,104,48, 99]), memory);
        assert_eq!(&48, buffer.last().unwrap());
        assert_eq!(State::Paused, computer.state);
        assert_eq!(4, computer.instruction_cursor);


        let (memory, buffer) = computer.run();
        assert_eq!(hashed_map_fill!(i64, i64, vec![104, -42,104,48, 99]), memory);
        assert_eq!(State::Stopped, computer.state);
        assert_eq!(5, computer.instruction_cursor);


        // init
        let program = vec![3,9,4,9,3,10,4,10,99,-1,-1];
        let mut computer = Computer::new(program.clone());
        computer.set_resume_mode(ResumeMode::Enable);


        // first step
        computer.add_input(12);
        let (memory, buffer) = computer.run();
        assert_eq!(memory, hashed_map_fill!(i64, i64, vec![3,9,4,9,3,10,4,10,99,12,-1]));
        assert_eq!(buffer.last().unwrap(), &12);
        assert_eq!(computer.state, State::Paused);
        assert_eq!(computer.instruction_cursor, 4);

        // second step
        computer.add_input(42);
        let (memory, buffer) = computer.run();
        assert_eq!(memory, hashed_map_fill!(i64, i64, vec![3,9,4,9,3,10,4,10,99,12,42]));
        assert_eq!(buffer.last().unwrap(), &42);
        assert_eq!(computer.state, State::Paused);
        assert_eq!(computer.instruction_cursor, 8);


        // halt
        let (memory, _) = computer.run();
        assert_eq!(memory, hashed_map_fill!(i64, i64, vec![3,9,4,9,3,10,4,10,99,12,42]));
        assert_eq!(computer.state, State::Stopped);
        assert_eq!(computer.instruction_cursor, 9);

    }

    #[test]
    fn test_able_to_read_non_existing_memory_address() {
        let program = vec![3, 100000000000000,4,100000000000000,99];
        let mut expected = hashed_map_fill!(i64, i64, program.clone());
        expected.insert(100000000000000, 42);
        let (memory, buffer) = computer(program, Some(VecDeque::from(vec![42])));
        assert_eq!(expected, memory);
        assert_eq!(*buffer.last().unwrap(), 42);
    }

    #[test]
    fn test_relative_mode() {
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let (_, buffer) = computer(program.clone(), None);
        assert_eq!(buffer, program)
    }

    #[test]
    fn test_able_to_add_two_numbers_immediate_value_and_stores_it_at_relative_memory_address() {
        // Intcode 21101 : add 4 + 5 and stores it a address (base+3) + @4 -> @7 = 42
        let program = vec![109,3,21101,4,38,4,99,7];
        let mut expected = hashed_map_fill!(i64, i64, program.clone());
        expected.insert(7, 42);
        let (memory, _) = computer(program, Some(VecDeque::from(vec![3])));
        assert_eq!(memory, expected);
    }

    #[test]
    fn test_able_to_store_at_relative_address() {
        // Intcode 203
        let program = vec![109,3,203,2,99,-1];
        let mut expected = hashed_map_fill!(i64, i64, program.clone());
        expected.insert(5, 42);
        let (memory, _) = computer(program, Some(VecDeque::from(vec![42])));
        assert_eq!(memory, expected);
    }
}

