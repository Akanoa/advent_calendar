use std::path::PathBuf;
use common::computer::{read_program_file, computer};
use std::collections::VecDeque;

pub fn part_1() -> i64 {
    let path = PathBuf::from("./assets/boost.txt");
    let memory = read_program_file(path).unwrap();
    let (_, buffer) = computer(memory, Some(VecDeque::from(vec![1])));
    buffer.last().unwrap().to_owned()
}