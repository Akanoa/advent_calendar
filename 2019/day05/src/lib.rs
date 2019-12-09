use std::path::PathBuf;
use common::computer::{read_program_file, computer};

pub fn part_1() -> Vec<i32> {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();

    let (_, outputs) = computer(memory, &mut Some(vec![1]));
    outputs
}

pub fn part_2() -> Vec<i32> {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();

    let (_, outputs) = computer(memory, &mut Some(vec![5]));
    outputs
}