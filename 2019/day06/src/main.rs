extern crate indextree;
extern crate common;

use day06::{part_1, part_2};

fn main() {
    let result1 = part_1();
    let result2 = part_2();
    println!("The sum orbits direct and indirect is {}", result1);
    println!("Minimum transfers to reach Santa is {}", result2);
}
