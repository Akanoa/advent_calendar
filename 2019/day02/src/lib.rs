use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use common::computer::{computer, read_program_file};

pub fn part_1(input_1: u32, input_2: u32) -> Vec<u32> {
    let path = PathBuf::from("./assets/program.txt");
    let mut memory = read_program_file(path).unwrap();
    // Define inputs
    memory[1] = input_1;
    memory[2] = input_2;
    // Compute
    computer(memory)
}

pub fn part_2(result: u32) -> Option<(u32, u32)> {

    let path = PathBuf::from("./assets/program.txt");
    let memory_template = read_program_file(path).unwrap();

    for i in 0..100 as u32 {
        for j in 0..100 as u32 {
            let mut memory = memory_template.clone();
            memory[1] = i;
            memory[2] = j;
            let memory = computer(memory);
            if memory[0] == result {
                return Some((i, j))
            }
        }
    }
    None
}