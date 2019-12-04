use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
mod macros {
    macro_rules! points_from_command {
        ( $( ($x:expr, $y:expr)), *) => {
            {
                let mut temp_vec : Vec<(i32, i32)> =  vec![(0, 0)];
                let mut previous_pos = (0, 0);
                $(
                    let (prev_x, prev_y) = previous_pos;
                    previous_pos = (prev_x + $x, prev_y + $y);
                    temp_vec.push(previous_pos);
                )*
                temp_vec
            }
        };
        ($vec:expr) => {
            {
                let mut temp_vec : Vec<(i32, i32)> =  vec![(0, 0)];
                let mut previous_pos = (0, 0);
                for (x, y) in $vec {
                    let (prev_x, prev_y) = previous_pos;
                    previous_pos = (prev_x + x, prev_y + y);
                    temp_vec.push(previous_pos);
                }
            temp_vec
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Wire {
    path: Vec<(i32, i32)>
}

struct Command;

impl Command {
    fn from_string(value: String) -> Option<(i32, i32)> {

        let unit_vector : Option<(i32, i32)> = match value.chars().next() {
            Some(direction) => {
              match direction {
                  'R' => Some((1, 0)),
                  'L' => Some((-1, 0)),
                  'U' => Some((0, 1)),
                  'D' => Some((0, -1)),
                  _ => panic!("Unknown direction")
              }
            },
            None => None
        };

        match unit_vector  {
            Some(unit_vector) => {
                let amount_string = value.chars().skip(1).collect::<String>();
                let amount = match amount_string.parse::<i32>() {
                    Ok(amount) => Some(amount),
                    Err(_err) => None
                };

                match amount {
                    Some(amount) => {
                        let (x, y) = unit_vector;
                        Some((x * amount, y * amount))
                    },
                    None => None
                }
            },
            None => None
        }
    }
    fn line_to_command_list(line: String) -> Vec<String> {
        line.split(',')
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
}

impl Wire {
    fn new(path: Vec<(i32, i32)>) -> Wire {
        Wire {
            path
        }
    }

    fn get_points(&self) -> Vec<(i32, i32)> {
        let path = self.path.clone();
        points_from_command![path]
    }
}


fn load_from_file(path: PathBuf) -> Result<Vec<Wire>, Box<dyn Error>>{
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let wires : Vec<Wire> = reader
        .lines()
        .map(|line| {
            let commands = Command::line_to_command_list(line.unwrap())
                .into_iter()
                .map(|command| {
                    Command::from_string(command)
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<(i32, i32)>>();
            commands
        })
        .map(|path| Wire::new(path))
        .collect();



    Ok(wires)
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{load_from_file, Wire, Command};

    #[test]
    fn test_load_from_file() {
        let path = PathBuf::from("./assets/configuration1.txt");
        let result = load_from_file(path).unwrap();
        assert_eq!(vec![
            Wire::new(vec![(8, 0), (0, 5), (-5, 0), (0, -3)]),
            Wire::new(vec![(0, 7), (6, 0), (0, -4), (-4, 0)])
        ],result)
    }

    #[test]
    fn test_direction_to_tuple() {
        assert_eq!(Some((8, 0)), Command::from_string("R8".to_string()));
        assert_eq!(Some((0, 5)), Command::from_string("U5".to_string()));
        assert_eq!(Some((-5, 0)), Command::from_string("L5".to_string()));
        assert_eq!(Some((0, -3)), Command::from_string("D3".to_string()));
    }

    #[test]
    fn test_line_to_command_list() {
        assert_eq!(vec!["R8","U5","L5","D3"], Command::line_to_command_list("R8,U5,L5,D3".to_string()))
    }

    #[test]
    fn test_path_to_point() {

        // test the macro
        let points = points_from_command![(1, 0), (0, 1)];
        assert_eq!(vec![(0, 0), (1, 0), (1, 1)], points);

        let points2 = points_from_command![vec![(1, 0), (0, 1)]];
        assert_eq!(vec![(0, 0), (1, 0), (1, 1)], points2);

        let points3 = Wire::new(vec![(1, 0), (0, 1)]).get_points();
        assert_eq!(vec![(0, 0), (1, 0), (1, 1)], points3);
    }
}