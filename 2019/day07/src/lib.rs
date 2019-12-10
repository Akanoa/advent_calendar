use permutator::{Combination, Permutation};
use common::computer::{computer, read_program_file};
use std::path::PathBuf;
use std::collections::VecDeque;

#[derive(Debug)]
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

    fn run(&mut self, input : i32) -> i32 {
        //println!("setting {}", self.setting);
        let (memory, mut buffer) = computer(self.program.clone(), Some(VecDeque::from(vec![self.setting, input])));
        self.program = memory;
        buffer.pop().unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
enum AmplifierChainMode {
    Linear,
    Feedback
}

#[derive(Debug)]
struct AmplifierChain {
    chain: Vec<Amplifier>,
    mode: AmplifierChainMode
}

impl AmplifierChain {

    fn new (settings : Vec<i32>, program: Vec<i32>, mode: AmplifierChainMode) -> AmplifierChain{

        let amplifier_a = Amplifier::new(*settings.get(0).unwrap(), program.clone());
        let amplifier_b = Amplifier::new(*settings.get(1).unwrap(), program.clone());
        let amplifier_c = Amplifier::new(*settings.get(2).unwrap(), program.clone());
        let amplifier_d = Amplifier::new(*settings.get(3).unwrap(), program.clone());
        let amplifier_e = Amplifier::new(*settings.get(4).unwrap(), program.clone());

        AmplifierChain {
            chain: vec![amplifier_a, amplifier_b, amplifier_c, amplifier_d, amplifier_e],
            mode
        }
    }

    fn run(&mut self, input: i32) -> i32 {
        match &mut self.mode {
            AmplifierChainMode::Linear => {
                self.run_linear(input)
            },
            AmplifierChainMode::Feedback => {
                self.run_feedback(input)
            },
        }
    }

    fn run_feedback(&mut self, input: i32) -> i32{
        let mut output = input;
        let mut amplifier_index = 0;
        let mut amplifier;
        loop {
            amplifier = self.chain.get_mut(amplifier_index).unwrap();
            println!("Amplifier {}", amplifier_index);
            output = amplifier.run(output);
            amplifier_index+=1;
            if amplifier_index > self.chain.len() {
                amplifier_index = 0
            }
        }
        output
    }

    fn run_linear(&mut self, input: i32) -> i32 {
        let mut output = input;
        for amplifier in &mut self.chain {
            output = amplifier.run(output);
        }
        output
    }
}


fn get_optimized_amplifier_chain(program: Vec<i32>, seed : &[i32], mode: AmplifierChainMode) -> i32 {
    let permutions = get_all_combinations_settings(seed);
    let mut max = 0;
    for permutation in permutions {
        let mut amplifier_chain = AmplifierChain::new(permutation, program.clone(), mode);
        let output = amplifier_chain.run(0);
        if output > max {
            max = output;
        }
    }
    max
}

pub fn part_1() -> i32 {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();
    get_optimized_amplifier_chain(memory, &[0,1,2,3,4], AmplifierChainMode::Linear)
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
    use crate::{get_all_combinations_settings, AmplifierChain, AmplifierChainMode, get_optimized_amplifier_chain};

    #[test]
    fn test_get_all_combiantion_settings() {
        let data = &[1, 2, 3, 4];
        assert_eq!(23, get_all_combinations_settings(data).len());
    }

    #[test]
    fn test_amplifier_chain() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let settings = vec![4,3,2,1,0];
        let mut amplifier_chains = AmplifierChain::new(settings, program, AmplifierChainMode::Linear);
        let result = amplifier_chains.run(0);
        assert_eq!(result, 43210);


        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                           101,5,23,23,1,24,23,23,4,23,99,0,0];
        let settings = vec![0,1,2,3,4];
        let mut amplifier_chains = AmplifierChain::new(settings, program, AmplifierChainMode::Linear);
        let result = amplifier_chains.run(0);
        assert_eq!(54321, result);

        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                           1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let settings = vec![1,0,4,3,2];
        let mut amplifier_chains = AmplifierChain::new(settings, program, AmplifierChainMode::Linear);
        let result = amplifier_chains.run(0);
        assert_eq!(65210, result);
    }

    #[test]
    fn test_amplifier_feedback_chain() {
//        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
//                           27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
//        let settings = vec![9,8,7,6,5];
//        let mut amplifier_chain = AmplifierChain::new(settings, program, AmplifierChainMode::Feedback);
//        println!("{:?}", amplifier_chain);
//        let result = amplifier_chain.run(0);
//        assert_eq!(result, 139629729);
    }

}