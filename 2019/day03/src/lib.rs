use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use common::hashed_fill;

#[macro_use]
mod macros {
    macro_rules! points_from_command {
        ( $( ($x:expr, $y:expr)), *) => {
            {
                let mut temp_vec : Vec<(i32, i32)> =  vec![];
                let mut previous_pos = (0, 0);
                $(
                    let (prev_x, prev_y) = previous_pos;

                    if $x != 0 {
                        for i in 1..($x as i32).abs() + 1 {
                            let new_pos = (prev_x + i * ($x as i32).signum(), prev_y);
                            temp_vec.push(new_pos);
                        }
                    }

                    if $y != 0 {
                        for i in 1..($y as i32).abs() + 1 {
                            let new_pos = (prev_x, prev_y + i * ($y as i32).signum());
                            temp_vec.push(new_pos);
                        }
                    }
                    previous_pos = (prev_x + $x, prev_y + $y);

                )*
                // the last previous_prev must be used, because after the last macro expansion this variable
                // becomes orphan
                drop(&previous_pos);
                temp_vec
            }
        };
        ($vec:expr) => {
            {
                let mut temp_vec : Vec<(i32, i32)> =  vec![];
                let mut previous_pos = (0, 0);
                for (x, y) in $vec {
                    let (prev_x, prev_y) = previous_pos;


                    if x != 0 {
                        for i in 1..(x as i32).abs() + 1 {
                            let new_pos = (prev_x + i * (x as i32).signum(), prev_y);
                            temp_vec.push(new_pos);
                        }
                    }

                    if y != 0 {
                        for i in 1..(y as i32).abs() + 1 {
                            let new_pos = (prev_x, prev_y + i * (y as i32).signum());
                            temp_vec.push(new_pos);
                        }
                    }

                    previous_pos = (prev_x + x, prev_y + y);
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

    fn intersect(&self, other: &Wire) -> Vec<(i32, i32)> {
        let set1 : HashSet<(i32, i32)> = hashed_fill!((i32, i32), self.get_points());
        let set2 : HashSet<(i32, i32)> = hashed_fill!((i32, i32), other.get_points());

        let mut result = vec![];
        for point in set1.intersection(&set2) {
            result.push(*point);
        }
        result
    }

    fn get_min_intersection_manhattan_distance(&self, other: &Wire) -> u32 {
        let intersections = self.intersect(&other);
        let distances = intersections.into_iter().map(|(x, y)| x.abs()+y.abs())
            .collect::<Vec<i32>>();

        let mut min  = distances[0];
        for d in distances {
            if d < min {
                min = d;
            }
        }
        min as u32
    }

    fn get_steps_from_command(&self, intersection: (i32, i32)) -> u32 {
        match self.get_points()
            .into_iter()
            .position(|point| point == intersection) {
                Some(pos) => (pos + 1) as u32,
                None => 0
            }

    }

    fn get_min_steps(&self, other: & Wire) -> Option<u32> {
        let wires = vec![self, other];
        let intersections = self.intersect(&other);

        let mut steps = vec![];

        for intersection in intersections {
            let temp_steps = wires.clone().into_iter()
                .map(|wire| wire.get_steps_from_command(intersection))
                .collect::<Vec<u32>>();
            steps.push(temp_steps);
        }

        let delays : Vec<u32> = steps.into_iter()
            .map(|intersection_steps_by_wires| {
                intersection_steps_by_wires.into_iter().sum()
            })
            .collect();

        match delays.len() != 0 {
            true => {

                let mut min = delays[0];
                for delay in delays.into_iter().skip(1) {
                    if delay < min {
                        min = delay
                    }
                }

                Some(min)
            },
            false => None
        }
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

pub fn part_1() -> u32{
    let path = PathBuf::from("./assets/configuration_prod.txt");
    let mut result = load_from_file(path).unwrap();
    let wire1 = result.pop().unwrap();
    let wire2 = result.pop().unwrap();

    wire1.get_min_intersection_manhattan_distance(&wire2)
}

pub fn part_2() -> Option<u32>{
    let path = PathBuf::from("./assets/configuration_prod.txt");
    let mut result = load_from_file(path).unwrap();
    let wire1 = result.pop().unwrap();
    let wire2 = result.pop().unwrap();

    wire1.get_min_steps(&wire2)
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use crate::{load_from_file, Wire, Command};
    use std::collections::HashSet;
    use super::*;

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
        let points = points_from_command![(1, 0), (0, -2)];
        assert_eq!(points, vec![(1, 0), (1, -1), (1, -2)]);

        let points2 = points_from_command![vec![(1, 0), (0, -2)]];
        assert_eq!(points2, vec![(1, 0), (1, -1), (1, -2)]);

        let points3 = Wire::new(vec![(1, 0), (0, 1)]).get_points();
        assert_eq!(vec![(1, 0), (1, 1)], points3);
    }

    #[test]
    fn test_intersection() {

        let wire1 = Wire::new(vec![(8, 0), (0, 5), (-5, 0), (0, -3)]);
        let wire2 = Wire::new(vec![(0, 7), (6, 0), (0, -4), (-4, 0)]);

        let intersections = self::hashed_fill!((i32, i32), wire1.intersect(&wire2));
        let expected_hashed = self::hashed_fill!((i32, i32), vec![(6,5), (3,3)]);
        let hash_intersection = expected_hashed.difference(&intersections);
        assert_eq!(hash_intersection.count(), 0);
    }

    #[test]
    fn test_get_min_intersection_manhattan_distance() {
        let wire1 = Wire::new(vec![(8, 0), (0, 5), (-5, 0), (0, -3)]);
        let wire2 = Wire::new(vec![(0, 7), (6, 0), (0, -4), (-4, 0)]);

        assert_eq!(wire1.get_min_intersection_manhattan_distance(&wire2), 6);

        let path = PathBuf::from("./assets/configuration1.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_intersection_manhattan_distance(&wire2), 6);

        let path = PathBuf::from("./assets/configuration2.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_intersection_manhattan_distance(&wire2), 159);

        let path = PathBuf::from("./assets/configuration3.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_intersection_manhattan_distance(&wire2), 135);
    }

    #[test]
    fn test_get_steps() {
        let wire = Wire::new(vec![(2, 0), (0, 4)]);
        let steps = wire.get_steps_from_command((2,2));
        assert_eq!(4, steps);

        let wire = Wire::new(vec![(2, 0), (0, 4)]);
        let steps = wire.get_steps_from_command((2,2));
        assert_eq!(steps, 4);

        let wire = Wire::new(vec![(8, 0), (0, 5), (-5, 0), (0, -3)]);
        let steps = wire.get_steps_from_command((6,5));
        assert_eq!(steps, 15);
        let steps = wire.get_steps_from_command((3,3));
        assert_eq!(steps, 20);

    }

    #[test]
    fn test_get_min_steps() {

        let path = PathBuf::from("./assets/configuration1.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_steps(&wire2), Some(30));

        let path = PathBuf::from("./assets/configuration2.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_steps(&wire2), Some(610));

        let path = PathBuf::from("./assets/configuration3.txt");
        let mut result = load_from_file(path).unwrap();
        let wire1 = result.pop().unwrap();
        let wire2 = result.pop().unwrap();

        assert_eq!(wire1.get_min_steps(&wire2), Some(410));
    }
}