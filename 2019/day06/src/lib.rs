use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn load_from_file(path: PathBuf) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let results = reader
        .lines()
        .map(|line| parse_identifiers(line.unwrap()))
        .collect::<Vec<(String, String)>>();

    Ok(results)
}

fn parse_identifiers(line: String) -> (String, String) {
    let mut result : Vec<String> = line.split(")").map(|x| x.to_string()).collect();

    if result.len() != 2 {
        panic!("Unable to parse the line {}", line);
    }

    let right = result.pop().unwrap();
    let left = result.pop().unwrap();

    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::{parse_identifiers, load_from_file};
    use std::path::PathBuf;

    #[macro_use]
    mod macros {
        macro_rules! vec_tuple_str_to_vec_tuple_string {
            ($vec:expr) => {
                $vec.into_iter()
                .map(|(left, right)| (left.to_string(), right.to_string()))
                .collect::<Vec<(String,String)>>()
            };
        }

        macro_rules! to_str {
            ($left:expr, $right:expr) => {
                {
                    ($left.to_string(), $right.to_string())
                }
            };
        }
    }

    #[test]
    fn test_parse_identifiers() {

        assert_eq!(parse_identifiers("A)B".to_string()), to_str!("A", "B"));
        assert_eq!(parse_identifiers("AA)AB".to_string()), to_str!("AA","AB"));
    }

    #[test]
    fn test_macro_vec_tuple_str_to_vec_tuple_string() {
        let  a = vec![("A", "B"), ("D", "E")];
        let b = vec![("A".to_string(), "B".to_string()), ("D".to_string(), "E".to_string())];
        assert_eq!(vec_tuple_str_to_vec_tuple_string!(a), b);
    }

    #[test]
    fn test_load_from_file() {
        let path = PathBuf::from("./assets/dev_program1.txt");
        let result = load_from_file(path).unwrap();
        assert_eq!(vec_tuple_str_to_vec_tuple_string!(vec![
            ("E", "F"),
            ("A", "B"),
            ("C", "D"),
            ("B", "E"),
            ("B", "C"),
            ("E", "G")
        ]), result);
    }
}