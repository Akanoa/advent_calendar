use std::path::PathBuf;
use common::computer::{computer, read_program_file};

pub fn part_1(input_1: u32, input_2: u32) -> Vec<i64> {
    let path = PathBuf::from("./assets/program.txt");
    let mut memory = read_program_file(path).unwrap();
    // Define inputs
    memory[1] = input_1 as i64;
    memory[2] = input_2 as i64;
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
            memory[1] = i as i64;
            memory[2] = j as i64;
            let (memory, _) = computer(memory, None);
            if memory[0] == result as i64{
                return Some((i, j))
            }
        }
    }
    None
}