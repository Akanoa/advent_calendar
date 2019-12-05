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

        instruction_cursor += 4;
    }

    memory
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
