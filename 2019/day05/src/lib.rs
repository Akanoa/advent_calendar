use std::path::PathBuf;
use common::computer::{read_program_file, computer};
use std::collections::VecDeque;

pub fn part_1() -> Vec<i64> {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();

    let (_, outputs) = computer(memory, Some(VecDeque::from(vec![1])));
    outputs
}

pub fn part_2() -> Vec<i64> {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();

    let (_, outputs) = computer(memory, Some(VecDeque::from(vec![5])));
    outputs
}