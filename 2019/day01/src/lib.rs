use std::fs::File;
use std::path::{PathBuf};
use std::error::Error;
use std::io::{BufReader, BufRead};

///
/// Returns the fuel required for a certain mass
/// using the relation fuel_required = round( mass / 3 ) - 2
///
fn compute_fuel_from_mass(mass: u32) -> u32 {
    (((mass / 3) as f64).round() - 2 as f64) as u32
}

///
/// Load a mass module file
/// Each line is the mass of one module
///
fn read_module_file(path: PathBuf) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let result : Vec<u32> = reader.lines()
        .map(|x| {
            match x {
                Ok(line) => {
                    match line.parse::<u32>() {
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

pub fn part_1() {
    let path = PathBuf::from("./assets/modules_mass_list.txt");
    let modules = read_module_file(path).unwrap();

    let result = modules
        .into_iter()
        .map(|module_mass| compute_fuel_from_mass(module_mass))
        .sum::<u32>();

    println!("{:?}", result)
}


#[cfg(test)]
mod tests {
    use crate::{compute_fuel_from_mass, read_module_file};
    use std::path::{PathBuf};

    #[test]
    fn test_compute_fuel_from_mass() {
        assert_eq!(compute_fuel_from_mass(12), 2, "The fuel required must be 2");
        assert_eq!(compute_fuel_from_mass(14), 2, "The fuel required must be 2");
        assert_eq!(compute_fuel_from_mass(1969), 654, "The fuel required must be 654");
        assert_eq!(compute_fuel_from_mass(100756), 33583, "The fuel required must be 654");
    }

    #[test]
    fn test_read_module_file() {
        let path = PathBuf::from("./assets/dev_example.txt");
        let results = read_module_file(path).unwrap();
        assert_eq!(results, vec![12,45,22])
    }
}