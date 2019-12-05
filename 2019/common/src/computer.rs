#[derive(PartialEq, Debug)]
pub enum OpCode {
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


pub fn computer(mut memory :Vec<u32>) -> Vec<u32> {

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

        instruction_cursor += OpCode::get_increment(opcode);
    }

    memory
}


#[cfg(test)]
mod tests {
    use super::{OpCode, computer};

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
    fn test_computer() {
        assert_eq!(computer(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99], "Must be able to add two numbers");
        assert_eq!(computer(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99], "Must be able to multiply two numbers");
        assert_eq!(computer(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801], "Must be able to multiply two numbers and store the result");
        assert_eq!(computer(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99], "Must be able to handle complex program");
    }
}