use permutator::{Combination, Permutation};
use common::computer::{computer, read_program_file};
use std::path::PathBuf;

struct Amplifier {
    setting: i32,
    program: Vec<i32>
}

impl Amplifier {
    fn new (setting : i32, program: Vec<i32>) -> Amplifier{
        Amplifier {
            setting,
            program
        }
    }

    fn run(&self, input : i32) -> i32 {
        let (_, mut buffer) = computer(self.program.clone(), &mut Some(vec![input, self.setting]));
        buffer.pop().unwrap()
    }
}


struct AmplifierChain {
    chain: Vec<Amplifier>
}

impl AmplifierChain {

    fn new (settings : Vec<i32>, program: Vec<i32>) -> AmplifierChain{

        let amplifier_a = Amplifier::new(*settings.get(0).unwrap(), program.clone());
        let amplifier_b = Amplifier::new(*settings.get(1).unwrap(), program.clone());
        let amplifier_c = Amplifier::new(*settings.get(2).unwrap(), program.clone());
        let amplifier_d = Amplifier::new(*settings.get(3).unwrap(), program.clone());
        let amplifier_e = Amplifier::new(*settings.get(4).unwrap(), program.clone());

        AmplifierChain {
            chain: vec![amplifier_a, amplifier_b, amplifier_c, amplifier_d, amplifier_e]
        }
    }

    fn run(&self, input: i32) -> i32 {
        let mut output = input;
        for amplifier in &self.chain {
            output = amplifier.run(output);
        }
        output
    }
}


fn get_optimized_amplifier_chain(program: Vec<i32>) -> i32 {
    let permutions = get_all_combinations_settings(&[0,1,2,3,4]);
    let mut max = 0;
    let mut counter = 0;
    println!("Permutations number {}", permutions.len());
    for permutation in permutions {
        println!("{:?}, {}", permutation, counter);
        let amplifier_chain = AmplifierChain::new(permutation, program.clone());
        let output = amplifier_chain.run(0);
        if output > max {
            max = output;
        }
        counter += 1;
    }
    max
}

pub fn part_1() -> i32 {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();
    get_optimized_amplifier_chain(memory)
}


fn get_all_combinations_settings(data : &[i32]) -> Vec<Vec<i32>> {
    let mut result = vec![];
    data.combination(data.len()).for_each(|mut c| {
        c.permutation().for_each(|p| {
            let tmp = p.into_iter().map(|x| *x).collect::<Vec<i32>>();
            result.push(tmp)
        });
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_all_combinations_settings, AmplifierChain};

    #[test]
    fn test_get_all_combiantion_settings() {
        let data = &[1, 2, 3, 4];
        assert_eq!(24, get_all_combinations_settings(data).len());
    }

    #[test]
    fn test_amplifier_chain() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let settings = vec![4,3,2,1,0];
        let amplifier_chains = AmplifierChain::new(settings, program);
        let result = amplifier_chains.run(0);
        assert_eq!(43210, result);


        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                           101,5,23,23,1,24,23,23,4,23,99,0,0];
        let settings = vec![0,1,2,3,4];
        let amplifier_chains = AmplifierChain::new(settings, program);
        let result = amplifier_chains.run(0);
        assert_eq!(54321, result);

        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                           1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let settings = vec![1,0,4,3,2];
        let amplifier_chains = AmplifierChain::new(settings, program);
        let result = amplifier_chains.run(0);
        assert_eq!(65210, result);
    }

}