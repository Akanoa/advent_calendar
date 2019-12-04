use day03;
use day03::{part_1, part_2};

fn main() {
    let result1 = part_1();
    let result2 = part_2();
    println!("The minimum manhattan distance is {}", result1);
    match result2 {
        Some(min_steps) => println!("The minimum steps between one intersection and the Origin is {}", min_steps),
        None => println!("No minimal steps found")
    }
}
