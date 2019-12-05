use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use common::computer::computer;

///
/// Loads program
///
///
fn read_program_file(path: PathBuf) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .split(',')
        .map(|x| {
            match x.parse::<u32>() {
                Ok(x) => Some(x),
                Err(_e) => None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();


    Ok(result)
}

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

#[cfg(test)]
mod tests {
    use crate::{read_program_file};
    use std::path::PathBuf;

    #[test]
    fn test_read_program_file() {
        let path = PathBuf::from("./assets/dev_program.txt");
        let results = read_program_file(path).unwrap();
        assert_eq!(results, vec![1,0,0,0,99], "Must read the right value from file")
    }
}