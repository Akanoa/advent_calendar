use day05::part_1;

fn main() {
    let output1 = part_1();
    println!("Starting diagnostic");
    for (step, code) in output1.into_iter().enumerate() {
        println!("Step #{} : {}", step, code);
    }
}
