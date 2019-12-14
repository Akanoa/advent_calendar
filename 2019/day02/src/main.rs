use day02::{part_1, part_2};

fn main() {
    let result_1 = part_1(12, 2);
    println!("The result of the program part 1 is {}", result_1);
    match part_2(19690720)  {
        Some((input_1, input_2)) => {
            println!("The noun={} and the verb={}", input_1, input_2);
            println!("Thus the answer is {}", input_1* 100 + input_2);
        },
        None => eprintln!("No input could be found to given output")
    };
}
