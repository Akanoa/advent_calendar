use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::error::Error;
use std::collections::HashMap;
use std::mem;
use std::f64::consts::PI;


/// Returns the mantissa, exponent and sign as integers.
fn integer_decode(x:f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(x) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}


#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    x: f64,
    y: f64
}

#[derive(PartialEq, Debug, Clone)]
pub struct Asteroid {
    pub position: Position
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Asteroid {
        Asteroid {
            position: Position {x, y},
        }
    }

    fn get_module_and_argument(&self, reference: &Asteroid) -> (f64, f64) {

        // axis system changement
        let (x, y) = (self.position.x - reference.position.x, self.position.y - reference.position.y);
        // calculate module
        let module = (x.powi(2) + y.powi(2)).sqrt();
        // calculate argument
        let argument = y.atan2(x);

        (module,argument)
    }
}

fn normalize_atan2(y: f64, x : f64) -> f64 {

    let mut theta = (x).atan2(y);
    if x < 0.0 {
        theta += 2.0 * PI;
    }

    theta
}

pub fn part_1() -> u64{
    let path = PathBuf::from("./assets/prod.txt");
    let asteroids = read_from_file(path).unwrap();
    let result = get_best_asteroid(asteroids);
    result.1
}

fn get_best_asteroid(asteroids: Vec<Asteroid>) -> (Position, u64) {

    let mut max = 0;
    let mut best_pos : Position = Position { x: 0.0, y: 0.0 };
    for asteroid in &asteroids {
        let number_asteroid_visible = get_number_asteroids_visible(&asteroid, asteroids.clone());
        if number_asteroid_visible > max {
            best_pos = asteroid.position.clone();
            max = number_asteroid_visible;
        }
    }

    (best_pos, max)
}

fn get_number_asteroids_visible(reference: &Asteroid, others: Vec<Asteroid>) -> u64 {

    let mut map_min_module_by_argument: HashMap<(u64, i16, i8), f64> = HashMap::new();

    // build the map
    for other in &others {

        if other == reference {
            continue
        }

        let (module, arg) = other.get_module_and_argument(reference);
        let angle = integer_decode(arg);
        match map_min_module_by_argument.get_mut(&angle) {
            None => {
                map_min_module_by_argument.insert(angle, module);
            },
            Some(min_module) => {
                if module < *min_module {
                    *min_module = module
                }
            },
        }
    }

    let mut count = 0;
    // filter hidden asteroids
    for other in &others {

        if other == reference {
            continue
        }

        let (module, arg) = other.get_module_and_argument(reference);
        let angle = integer_decode(arg);
        let min_module = map_min_module_by_argument.get(&angle).unwrap();
        if module == *min_module {
            count+=1;
        }

    }

    count
}


fn read_from_file(path: PathBuf) -> Result<Vec<Asteroid>, Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut x : f64;
    let mut y: f64 = 0f64;
    let mut result = vec![];

    for line in reader.lines() {
        x=0f64;
        for c in line.unwrap().chars() {
            if c == '#' {
                result.push(Asteroid::new(x, y));
            }
            x+=1f64
        }
        y+=1f64
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{Asteroid, read_from_file, get_number_asteroids_visible, get_best_asteroid, Position, normalize_atan2};
    use std::f64::consts::{PI, FRAC_PI_4, FRAC_PI_2};

    #[test]
    fn test_read_from_file() {
        let path = PathBuf::from("./assets/simple_example.txt");
        let expected = vec![
            Asteroid::new(0f64,0f64),
            Asteroid::new(2f64,0f64),
            Asteroid::new(3f64,0f64),
            Asteroid::new(4f64,1f64)
        ];
        let result = read_from_file(path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_module_and_argument() {
        let reference = Asteroid::new(0f64,0f64);

        let target = Asteroid::new(1f64,0f64);
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 1f64);
        assert_eq!(arg, 0f64);

        let target = Asteroid::new(0f64,1f64);
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 1f64);
        assert_eq!(arg, FRAC_PI_2);

        let target = Asteroid::new(0f64,-1f64);
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 1f64);
        assert_eq!(arg, -FRAC_PI_2, "must be able to target the right quadrant");

        let target = Asteroid::new(-1f64,0f64);
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 1f64);
        assert_eq!(arg, PI);

        let target = Asteroid::new(-2f64,0f64);
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 2f64, "must be able to calculate the right module");
        assert_eq!(arg, PI);

        let target = Asteroid::new(2f64 * FRAC_PI_4.cos(),2f64 * FRAC_PI_4.sin());
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 2f64, "must be able to calculate the right module");
        assert_eq!(arg, FRAC_PI_4);

        let reference = Asteroid::new(1f64,1f64);
        let target = Asteroid::new(2f64 * FRAC_PI_4.cos(),2f64 * FRAC_PI_4.sin());
        let (module, arg) = target.get_module_and_argument(&reference);
        assert_eq!(module, 0.5857864376269051_f64, "must be able to calculate the right module");
        assert_eq!(arg, FRAC_PI_4);
    }

    #[test]
    fn test_filter_visible_asteroids() {
        let reference = Asteroid::new(0.5, 0.5);
        let target_a = Asteroid::new( FRAC_PI_4.cos(),FRAC_PI_4.sin());
        let target_b = Asteroid::new( 3f64 * FRAC_PI_4.cos(),3f64 * FRAC_PI_4.sin());
        let target_c = Asteroid::new(0f64, 1f64);

        let targets = vec![target_a, target_b, target_c];
        let result = get_number_asteroids_visible(&reference, targets);
        assert_eq!(result, 2);

    }

    #[test]
    fn test_get_best_asteroid() {
        let path = PathBuf::from("./assets/dev_example_33.txt");
        let asteroids = read_from_file(path).unwrap();
        let result = get_best_asteroid(asteroids);
        assert_eq!(result, (Position { x: 5.0, y:8.0 }, 33));

        let path = PathBuf::from("./assets/dev_example_35.txt");
        let asteroids = read_from_file(path).unwrap();
        let result = get_best_asteroid(asteroids);
        assert_eq!(result, (Position { x: 1.0, y:2.0 }, 35));

        let path = PathBuf::from("./assets/dev_example_41.txt");
        let asteroids = read_from_file(path).unwrap();
        let result = get_best_asteroid(asteroids);
        assert_eq!(result, (Position { x: 6.0, y:3.0 }, 41));

        let path = PathBuf::from("./assets/dev_example_210.txt");
        let asteroids = read_from_file(path).unwrap();
        let result = get_best_asteroid(asteroids);
        assert_eq!(result, (Position { x: 11.0, y:13.0 }, 210));
    }

    #[test]
    fn test_normalize_atan2() {
        assert_eq!(normalize_atan2(1.0,0.0), 0.0);
        assert_eq!(normalize_atan2(0.0,1.0), FRAC_PI_2);
        assert_eq!(normalize_atan2(-1.0,0.0), PI);
        assert_eq!(normalize_atan2(0.0,-1.0), 3f64 * FRAC_PI_2);
        assert_eq!(normalize_atan2(-0.8,-0.6), 3.7850937623830774);
        assert_eq!(normalize_atan2(0.6,0.8), 0.9272952180016123);
        assert_eq!(normalize_atan2(1.2,1.6), 0.9272952180016123);
        assert_eq!(normalize_atan2(-0.8,0.6), 2.498091544796509);
        assert_eq!(normalize_atan2(-0.8,-0.6), 3.7850937623830774);
        assert_eq!(normalize_atan2(0.6,-0.8), 5.355890089177974);
    }
}