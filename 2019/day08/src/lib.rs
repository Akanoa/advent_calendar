use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::collections::HashMap;

fn get_layers_from_string(input: String, image_size: u32) -> Vec<Vec<char>> {
    let mut output = vec![];
    let mut tmp = vec![];
    let mut cursor = 0;

    let chars = input.chars();
    for c in chars {
        tmp.push(c);
        cursor+=1;
        if cursor % image_size == 0 {
            output.push(tmp);
            tmp = vec![];
        }
    }

    output
}

fn read_from_file(path: PathBuf) -> Result<String, Box<dyn Error>> {

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    Ok(buffer)
}

fn get_count_by_elements(vec : Vec<char>) -> HashMap<String, u32>{
    let mut map : HashMap<String, u32> = HashMap::new();
    for c in vec {
        if !map.contains_key(&c.to_string()[..]) {
            map.insert(c.to_string(), 0);
        }
        *map.get_mut(&c.to_string()[..]).unwrap() +=1;
    }
    map
}

pub fn part_1() -> u32 {
    let path = PathBuf::from("./assets/prod.txt");
    let input = read_from_file(path).unwrap();
    let layers = get_layers_from_string(input, 25*6);

    let mut best_map = get_count_by_elements(layers[0].clone());
    let mut lowest_0  = *best_map.get("0").unwrap();

    for layer in layers {
        let map_layer = get_count_by_elements(layer.clone());
        let nb_0 = map_layer.get("0").unwrap();
        if nb_0 < &lowest_0 {
            best_map = map_layer.clone();
            lowest_0 = *nb_0;
        }
    }


    let nb_2 = &best_map.get("2").unwrap();
    let nb_1 = &best_map.get("1").unwrap();
    (*nb_2 * *nb_1) as u32
}


#[cfg(test)]
mod tests {
    use crate::{get_layers_from_string, read_from_file, get_count_by_elements};
    use std::path::PathBuf;

    #[test]
    fn test_get_layers_from_string() {
        let expected = vec![
            vec!['1','2','3','4','5','6'],
            vec!['7','8','9','0','1','2']
        ];
        let input = "123456789012".to_string();
        let result = get_layers_from_string(input, 3*2);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_read_from_file() {
        let path = PathBuf::from("./assets/dev.txt");
        let result = read_from_file(path).unwrap();

        assert_eq!(result, "123456789012".to_string());
    }

    #[test]
    fn test_get_counts_by_elements() {
        let input : Vec<char> = "102101102".chars().collect();
        let map = get_count_by_elements(input);
        let nb_2 = map.get("2").unwrap();
        let nb_1 = map.get("1").unwrap();
        let nb_0 = map.get("0").unwrap();
        assert_eq!(nb_2, &2);
        assert_eq!(nb_1, &4);
        assert_eq!(nb_0, &3);
    }
}