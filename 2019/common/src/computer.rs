use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::VecDeque;


#[macro_use]
mod macros {
    macro_rules! get_operand {
        ($memory:expr, $memory_address:expr, $instruction_cursor:expr, $parameters_mode:expr, $text_error:expr) => {

            match $parameters_mode {
                Mode::Immediate => {
                    match $memory.get($instruction_cursor + $memory_address as usize) {
                        Some(&x) => x,
                        None => panic!("{} (get immediate value): This memory address doesn't exist", $text_error)
                    }
                },
                Mode::Positional => {
                    let operand_address= match $memory.get($instruction_cursor + $memory_address as usize) {
                        Some(&x) => x,
                        None => panic!("{} (get memory address): This memory address doesn't exist", $text_error)
                    };

                    match $memory.get(operand_address as usize) {
                        Some(&x) => x,
                        None => panic!("{} (get value from memory address): This memory address doesn't exist", $text_error)
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
    Equals
}

impl OpCode {
    fn get_increment(opcode: OpCode) -> usize {
        match opcode {
            OpCode::Add | OpCode::Multiply | OpCode::LessThan | OpCode::Equals => 4,
            OpCode::Store | OpCode::Output => 2,
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
            "99" => OpCode::Stop,
            _ => panic!("Unknown opcode {}", string)
        }
    }

    fn get_opcode_and_modes_from_str(string: i32) -> (Parameter, OpCode) {

        // sanitize input
        let input = format!("{:0>4}", string)
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
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Store,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
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
    memory: Vec<i32>,
    output_buffer: Vec<i32>,
    input_data: Option<VecDeque<i32>>,
    pub resume_mode: ResumeMode,
    pub state: State,
    instruction_cursor: usize
}

impl Computer {
    pub fn new (program: Vec<i32>) -> Computer {
        Computer {
            memory: program,
            output_buffer: vec![],
            input_data: None,
            resume_mode: ResumeMode::Disable,
            state: State::Started,
            instruction_cursor: 0
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

    pub fn add_input(&mut self, input : i32) {
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

    pub fn run(&mut self) -> (Vec<i32>, Vec<i32>) {

        self.state = State::Started;

        loop {

            let increment;

            let opcode_raw =  match self.memory.get(self.instruction_cursor + Command::OpCode as usize) {
                Some(x) => x,
                None => panic!("OPCODE: This memory address doesn't exist")
            };

            let (parameters_mode, opcode) = OpCode::get_opcode_and_modes_from_str(*opcode_raw);

            if opcode == OpCode::Stop {
                self.state = State::Stopped;
                self.instruction_cursor += 1;
                break;
            };


            match opcode {
                OpCode::Add | OpCode::Multiply => {

                    let operand_1 : i32 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "OPERAND1");
                    let operand_2 : i32 = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "OPERAND2");

                    let result = match opcode {
                        OpCode::Add => {
                            operand_1 + operand_2
                        },
                        OpCode::Multiply => operand_1 * operand_2,
                        _ => panic!("Unknown opcode")
                    };

                    let store_address = get_operand!(self.memory, Command::ResultAddress, self.instruction_cursor, Mode::Immediate, "RESULT");
                    self.memory[store_address as usize] = result;
                    increment = OpCode::get_increment(opcode);

                },
                OpCode::Output | OpCode::Store => {
                    let address : i32 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, Mode::Immediate, "ADRRESS");
                    increment = OpCode::get_increment(opcode);
                    match opcode {
                        OpCode::Store => {
                            match &mut self.input_data {
                                Some(x) => {
                                    match &mut x.pop_front() {
                                        Some(data) => {
                                            self.memory[address as usize] = *data;
                                        },
                                        None => panic!("Unable to get value from input Vec")
                                    }
                                },
                                None => panic!("Unable get value to store")
                            };
                        },
                        OpCode::Output => {
                            let value : i32 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "ADRRESS");
                            self.output_buffer.push(value);

                            match self.resume_mode {
                                ResumeMode::Enable => {
                                    self.state = State::Paused;
                                },
                                ResumeMode::Disable => {},
                            }
                        },
                        _ => panic!("Unknown opcode")
                    }

                },
                OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                    let value_checked : i32 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "VALUE CHECKED");
                    let next_cursor_address : i32 = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "NEXT CURSOR ADDRESS");
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

                        self.instruction_cursor = next_cursor_address as usize;
                        increment = 0;
                    } else {
                        increment = OpCode::get_increment(opcode);
                    }
                },
                OpCode::Equals |OpCode::LessThan => {

                    let operand_1 : i32 = get_operand!(self.memory, Command::OperandAddress1, self.instruction_cursor, parameters_mode.first_operand, "OPERAND 1");
                    let operand_2 : i32 = get_operand!(self.memory, Command::OperandAddress2, self.instruction_cursor, parameters_mode.second_operand, "OPERAND 2");
                    let result_address : i32 = get_operand!(self.memory, Command::ResultAddress, self.instruction_cursor, Mode::Immediate, "RESUTLT ADDRESS");

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

                    self.memory[result_address as usize] = result;
                    increment = OpCode::get_increment(opcode);
                }
                _ => panic!("Unknown opcode")
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
pub fn read_program_file(path: PathBuf) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .split(',')
        .map(|x| match x.parse::<i32>() {
            Ok(value) => Some(value),
            Err(_err) => None
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    Ok(result)
}

pub fn computer(memory :Vec<i32>, input: Option<VecDeque<i32>>) -> (Vec<i32>, Vec<i32>) {

    let mut computer = Computer::new(memory);
    computer.input_data = input;

    computer.run()
}

pub fn computer_feedback(memory :Vec<i32>, input: Option<VecDeque<i32>>) -> (Vec<i32>, Vec<i32>) {

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

        let empty: Vec<i32> = Vec::new();

        assert_eq!(computer( vec![1, 0, 0, 0, 99], None),
                   (vec![2, 0, 0, 0, 99], empty.clone()), "Must be able to add two numbers");
        assert_eq!(computer(vec![2, 3, 0, 3, 99], None),
                   (vec![2, 3, 0, 6, 99], empty.clone()), "Must be able to multiply two numbers");
        assert_eq!(computer(vec![2, 4, 4, 5, 99, 0], None),
                   (vec![2, 4, 4, 5, 99, 9801], empty.clone()), "Must be able to multiply two numbers and store the result");
        assert_eq!(computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None),
                   (vec![30, 1, 1, 4, 2, 5, 6, 0, 99], empty.clone()), "Must be able to handle complex program");
        assert_eq!(computer(vec![1101, 100, -1, 4, 0], None),
                   (vec![1101, 100, -1, 4, 99], empty.clone()), "Can handle operation immediate value");
        assert_eq!(computer(vec![3, 0, 4, 0, 99], Some(VecDeque::from(vec![-42]))),
                   (vec![-42, 0, 4, 0, 99], vec![-42]), "Able to write in output buffer");

        // --- Positional mode
        // equality
        assert_eq!(computer(vec![3,9,8,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![8]))),
                   (vec![3,9,8,9,10,9,4,9,99,1,8], vec![1]), "Able to deal with equality (position mode)");
        assert_eq!(computer(vec![3,9,8,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![12]))),
                   (vec![3,9,8,9,10,9,4,9,99,0,8], vec![0]), "Able to deal with non equality (position mode)");
        // less than
        assert_eq!(computer(vec![3,9,7,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![5]))),
                   (vec![3,9,7,9,10,9,4,9,99,1,8], vec![1]), "Able to deal with less than (position mode)");
        assert_eq!(computer(vec![3,9,7,9,10,9,4,9,99,-1,8], Some(VecDeque::from(vec![12]))),
                   (vec![3,9,7,9,10,9,4,9,99,0,8], vec![0]), "Able to deal with greater than (position mode)");

        // --- Immediate mode
        // equality
        assert_eq!(computer(vec![3,3,1108,-1,8,3,4,3,99], Some(VecDeque::from(vec![8]))),
                   (vec![3,3,1108,1,8,3,4,3,99], vec![1]), "Able to deal with equality (immediate mode)");
        assert_eq!(computer(vec![3,3,1108,-1,8,3,4,3,99], Some(VecDeque::from(vec![12]))),
                   (vec![3,3,1108,0,8,3,4,3,99], vec![0]), "Able to deal with non equality (immediate mode)");
        // less than
        assert_eq!(computer(vec![3,3,1107,-1,8,3,4,3,99], Some(VecDeque::from(vec![5]))),
                   (vec![3,3,1107,1,8,3,4,3,99], vec![1]), "Able to deal with less than (immediate mode)");
        assert_eq!(computer(vec![3,3,1107,-1,8,3,4,3,99], Some(VecDeque::from(vec![12]))),
                   (vec![3,3,1107,0,8,3,4,3,99], vec![0]), "Able to deal with greater than (immediate mode)");

        // --- Positional mode
        assert_eq!(computer(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], Some(VecDeque::from(vec![1]))),
                   (vec![3,12,6,12,15,1,13,14,13,4,13,99,1,1,1,9], vec![1]), "Should jump if input 1 (position mode)");
        assert_eq!(computer(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], Some(VecDeque::from(vec![0]))),
                   (vec![3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9], vec![0]), "Should jump if input 0 (position mode)");

        // --- Immediate mode
        assert_eq!(computer(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Some(VecDeque::from(vec![1]))),
                   (vec![3,3,1105,1,9,1101,0,0,12,4,12,99,1], vec![1]), "Should jump if input 1 (immediate mode)");
        // --- Positional mode
        assert_eq!(computer(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], Some(VecDeque::from(vec![0]))),
                   (vec![3,3,1105,0,9,1101,0,0,12,4,12,99,0], vec![0]), "Should jump if input 0 (immediate mode)");

        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                           1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                           999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];


        // equal 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![8])));
        let expected : Vec<i32> = vec![1000];
        assert_eq!(buffer, expected);

        // greater than 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![220])));
        let expected : Vec<i32> = vec![1001];
        assert_eq!(buffer, expected);

        // less than 8
        let (_, buffer) = computer(program.clone(), Some(VecDeque::from(vec![7])));
        let expected : Vec<i32> = vec![999];
        assert_eq!(buffer, expected);


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
        assert_eq!(OpCode::get_opcode_and_modes_from_str(1),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional }, OpCode::Add));
        assert_eq!(OpCode::get_opcode_and_modes_from_str(1002),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Immediate }, OpCode::Multiply));
        assert_eq!(OpCode::get_opcode_and_modes_from_str(99),
                   (Parameter { first_operand: Mode::Positional, second_operand: Mode::Positional }, OpCode::Stop));
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
        assert_eq!(vec![3,5,3,6,99,220,-42], memory);

    }


    #[test]
    fn test_resumable_computer() {

        let program = vec![104, -42,104,48, 99];
        let mut computer = Computer::new(program.clone());
        computer.set_resume_mode(ResumeMode::Enable);
        let (memory, buffer) = computer.run();
        assert_eq!(vec![104,-42,104,48, 99], memory);
        assert_eq!(&-42, buffer.last().unwrap());
        assert_eq!(State::Paused, computer.state);
        assert_eq!(2, computer.instruction_cursor);


        let (memory, buffer) = computer.run();
        assert_eq!(vec![104, -42,104,48, 99], memory);
        assert_eq!(&48, buffer.last().unwrap());
        assert_eq!(State::Paused, computer.state);
        assert_eq!(4, computer.instruction_cursor);


        let (memory, buffer) = computer.run();
        assert_eq!(vec![104, -42,104,48, 99], memory);
        assert_eq!(State::Stopped, computer.state);
        assert_eq!(5, computer.instruction_cursor);


        // init
        let program = vec![3,9,4,9,3,10,4,10,99,-1,-1];
        let mut computer = Computer::new(program.clone());
        computer.set_resume_mode(ResumeMode::Enable);


        // first step
        computer.add_input(12);
        let (memory, buffer) = computer.run();
        assert_eq!(memory, vec![3,9,4,9,3,10,4,10,99,12,-1]);
        assert_eq!(buffer.last().unwrap(), &12);
        assert_eq!(computer.state, State::Paused);
        assert_eq!(computer.instruction_cursor, 4);

        // second step
        computer.add_input(42);
        let (memory, buffer) = computer.run();
        assert_eq!(memory, vec![3,9,4,9,3,10,4,10,99,12,42]);
        assert_eq!(buffer.last().unwrap(), &42);
        assert_eq!(computer.state, State::Paused);
        assert_eq!(computer.instruction_cursor, 8);


        // halt
        let (memory, _) = computer.run();
        assert_eq!(memory, vec![3,9,4,9,3,10,4,10,99,12,42]);
        assert_eq!(computer.state, State::Stopped);
        assert_eq!(computer.instruction_cursor, 9);

    }
}

