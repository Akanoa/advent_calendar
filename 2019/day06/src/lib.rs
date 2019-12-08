use std::path::PathBuf;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use indextree::{Arena, NodeId};
use std::collections::HashMap;

#[macro_use]
mod macros {
    macro_rules! get_node {
        ($side:expr, $index_node_map:ident, $arena:ident) => {

            match $index_node_map.get(&$side[..]) {
                Some(&node_id) => {
                    node_id
                },
                None => {
                    let node_id = $arena.new_node($side.clone());
                    $index_node_map.insert($side, node_id);
                    node_id
                }
            }

        };
    }
}

fn build_tree(configuration:Vec<(String, String)>) -> (Arena<String>, HashMap<String, NodeId>){

    let arena = &mut Arena::new();
    let mut index_node_map : HashMap<String, NodeId> = HashMap::new();

    for (parent, child) in configuration {

        let parent_node : NodeId  = get_node!(parent.clone(), index_node_map, arena);
        let child_node :NodeId = get_node!(child.clone(), index_node_map, arena);

        parent_node.append(child_node, arena)

    }

    (arena.to_owned(), index_node_map)
}

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
    use crate::{parse_identifiers, load_from_file, build_tree};
    use std::path::PathBuf;
    use indextree::Node;

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

        macro_rules! get_node_children_name {
            ($parent_name:expr, $tree:ident, $map:ident) => {
                {
                    let node_id = $map.get($parent_name).unwrap().to_owned();
                    let mut children_names = vec![];
                    for child_node_id in node_id.children(&$tree) {
                        let child_name = $tree.get(child_node_id).unwrap().get();
                        children_names.push(child_name);
                    }
                    children_names
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

    #[test]
    fn test_build_tree() {
        let configuration = vec_tuple_str_to_vec_tuple_string!(vec![
            ("B", "C"),
            ("A", "D"),
            ("A", "B"),
            ("A", "E")
        ]);

        let empty : Vec<&String> = Vec::new();
        let (tree, map) = build_tree(configuration);
        let  children_names = get_node_children_name!("A", tree, map);
        assert_eq!(vec!["D", "B", "E"], children_names);
        let  children_names = get_node_children_name!("B", tree, map);
        assert_eq!(vec!["C"], children_names);
        let  children_names = get_node_children_name!("C", tree, map);
        assert_eq!(empty, children_names);
        let  children_names = get_node_children_name!("D", tree, map);
        assert_eq!(empty, children_names);

    }
}