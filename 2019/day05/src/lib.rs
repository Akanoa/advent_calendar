use std::path::PathBuf;
use common::computer::{read_program_file, computer};

pub fn part_1() -> Vec<String> {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();

    let (_, outputs) = computer(memory, Some(1));
    outputs
}