use std::path::PathBuf;
use common::computer::{computer, read_program_file};

pub fn part_1(input_1: u32, input_2: u32) -> Vec<String> {
    let path = PathBuf::from("./assets/program.txt");
    let mut memory = read_program_file(path).unwrap();
    // Define inputs
    memory[1] = input_1.to_string();
    memory[2] = input_2.to_string();
    // Compute
    let (memory, _) = computer(memory, None);
    memory
}

pub fn part_2(result: u32) -> Option<(u32, u32)> {

    let path = PathBuf::from("./assets/program.txt");
    let memory_template = read_program_file(path).unwrap();

    for i in 0..100 as u32 {
        for j in 0..100 as u32 {
            let mut memory = memory_template.clone();
            memory[1] = i.to_string();
            memory[2] = j.to_string();
            let (memory, _) = computer(memory, None);
            if memory[0] == result.to_string() {
                return Some((i, j))
            }
        }
    }
    None
}