use day05::{part_1, part_2};

fn main() {
    let output1 = part_1();
    let output2 = part_2();
    println!("Starting diagnostic");
    for (step, code) in output1.into_iter().enumerate() {
        println!("Step #{} : {}", step, code);
    }
    println!("Diagnostic code to 5 ID:");
    for (step, code) in output2.into_iter().enumerate() {
        println!("Step #{} : {}", step, code);
    }
}
