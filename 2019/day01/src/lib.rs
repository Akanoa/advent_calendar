use std::fs::File;
use std::path::{PathBuf};
use std::error::Error;
use std::io::{BufReader, BufRead};

///
/// Returns the fuel required for a certain mass
/// using the relation fuel_required = round( mass / 3 ) - 2
///
fn compute_fuel_from_mass(mass: i32) -> i32 {
    (((mass / 3) as f64).round() - 2 as f64) as i32
}

///
/// Compute recursively the amount of
/// fuel needed to launch the module by taking in account the fuel
/// needed to the fuel to be launched
///
fn recursive_compute_fuel_amount(mass: i32) -> i32 {
    let result = compute_fuel_from_mass(mass);
    if result < 0 {
        return 0
    }
    let fuel = recursive_compute_fuel_amount(result);
    result + fuel
}

///
/// Load a mass module file
/// Each line is the mass of one module
///
fn read_module_file(path: PathBuf) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let result : Vec<i32> = reader.lines()
        .map(|x| {
            match x {
                Ok(line) => {
                    match line.parse::<i32>() {
                        Ok(value) => Some(value),
                        Err(_err) => None
                    }
                },
                Err(_err) => None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    Ok(result)
}

pub fn part_1() -> i32{
    let path = PathBuf::from("./assets/modules_mass_list.txt");
    let modules = read_module_file(path).unwrap();

    let result = modules
        .into_iter()
        .map(|module_mass| compute_fuel_from_mass(module_mass))
        .sum::<i32>();

    result
}


pub fn part_2() -> i32{
    let path = PathBuf::from("./assets/modules_mass_list.txt");
    let modules = read_module_file(path).unwrap();

    let result = modules
        .into_iter()
        .map(|module_mass| recursive_compute_fuel_amount(module_mass))
        .sum::<i32>();

    result
}


#[cfg(test)]
mod tests {
    use crate::{compute_fuel_from_mass, read_module_file, recursive_compute_fuel_amount};
    use std::path::{PathBuf};

    #[test]
    fn test_compute_fuel_from_mass() {
        assert_eq!(compute_fuel_from_mass(12), 2, "The fuel required must be 2");
        assert_eq!(compute_fuel_from_mass(14), 2, "The fuel required must be 2");
        assert_eq!(compute_fuel_from_mass(1969), 654, "The fuel required must be 654");
        assert_eq!(compute_fuel_from_mass(100756), 33583, "The fuel required must be 654");
    }

    #[test]
    fn test_recursive_compute_fuel_from_mass() {
        debug_assert_eq!(recursive_compute_fuel_amount(14), 2, "The fuel required must be 2");
        assert_eq!(recursive_compute_fuel_amount(1969), 966, "The fuel required must be 966");
        assert_eq!(recursive_compute_fuel_amount(100756), 50346, "The fuel required must be 50346");
    }

    #[test]
    fn test_read_module_file() {
        let path = PathBuf::from("./assets/dev_example.txt");
        let results = read_module_file(path).unwrap();
        assert_eq!(results, vec![12,45,22], "Must read the right value from file")
    }
}