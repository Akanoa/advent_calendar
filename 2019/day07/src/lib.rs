use permutator::{Combination, Permutation};
use common::computer::{read_program_file, Computer, ResumeMode, State};
use std::path::PathBuf;

#[derive(Debug)]
struct Amplifier {
    setting: i64,
    computer: Computer,
    mode: ResumeMode
}

impl Amplifier {
    fn new (setting : i64, program: Vec<i64>, mode: ResumeMode) -> Amplifier{

        let mut computer = Computer::new(program);
        computer.set_resume_mode(mode);

        Amplifier {
            setting,
            computer,
            mode
        }
    }

    fn run(&mut self, input : i64) -> i64 {

        // Put first setting
        if !self.computer.is_setup() {
            self.computer.add_input(self.setting)
        }
        self.computer.add_input(input);

        let (_, buffer) = self.computer.run();
        match buffer.last() {
            Some(x) => {
                x.to_owned()
            },
            None => panic!("Unable to get buffer value")
        }
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

    fn new (settings : Vec<i64>, program: Vec<i64>, mode: AmplifierChainMode) -> AmplifierChain {

        let resume_mode = match mode {
            AmplifierChainMode::Linear => ResumeMode::Disable,
            AmplifierChainMode::Feedback => ResumeMode::Enable
        };

        let amplifier_a = Amplifier::new(*settings.get(0).unwrap(), program.clone(), resume_mode);
        let amplifier_b = Amplifier::new(*settings.get(1).unwrap(), program.clone(), resume_mode);
        let amplifier_c = Amplifier::new(*settings.get(2).unwrap(), program.clone(), resume_mode);
        let amplifier_d = Amplifier::new(*settings.get(3).unwrap(), program.clone(), resume_mode);
        let amplifier_e = Amplifier::new(*settings.get(4).unwrap(), program.clone(), resume_mode);

        AmplifierChain {
            chain: vec![amplifier_a, amplifier_b, amplifier_c, amplifier_d, amplifier_e],
            mode
        }
    }

    fn run(&mut self, input: i64) -> i64 {
        match &mut self.mode {
            AmplifierChainMode::Linear => {
                self.run_linear(input)
            },
            AmplifierChainMode::Feedback => {
                self.run_feedback(input)
            },
        }
    }

    fn run_feedback(&mut self, input: i64) -> i64{
        let mut output = input;
        let mut amplifier_index = 0;
        let mut amplifier;
        loop {

            amplifier = self.chain.get_mut(amplifier_index).unwrap();

            if amplifier.computer.state == State::Stopped {
                break
            }

            output = amplifier.run(output);

            amplifier_index+=1;
            if amplifier_index > self.chain.len() - 1 {
                amplifier_index = 0
            }
        }
        output
    }

    fn run_linear(&mut self, input: i64) -> i64 {
        let mut output = input;
        for amplifier in &mut self.chain {
            output = amplifier.run(output);
        }
        output
    }
}


fn get_optimized_amplifier_chain(program: Vec<i64>, seed : &[i64], mode: AmplifierChainMode) -> i64 {
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

pub fn part_1() -> i64 {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();
    get_optimized_amplifier_chain(memory, &[0,1,2,3,4], AmplifierChainMode::Linear)
}

pub fn part_2() -> i64 {
    let path = PathBuf::from("./assets/program.txt");
    let memory = read_program_file(path).unwrap();
    get_optimized_amplifier_chain(memory, &[5,6,7,8,9], AmplifierChainMode::Feedback)
}


fn get_all_combinations_settings(data : &[i64]) -> Vec<Vec<i64>> {
    let mut result = vec![];
    data.combination(data.len()).for_each(|mut c| {
        c.permutation().for_each(|p| {
            let tmp = p.into_iter().map(|x| *x).collect::<Vec<i64>>();
            result.push(tmp)
        });
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::{get_all_combinations_settings, AmplifierChain, AmplifierChainMode};

    #[test]
    fn test_get_all_combination_settings() {
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
        let program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                           27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let settings = vec![9,8,7,6,5];
        let mut amplifier_chain = AmplifierChain::new(settings, program, AmplifierChainMode::Feedback);
        let result = amplifier_chain.run(0);
        assert_eq!(result, 139629729);


        let program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                           -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                           53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let settings = vec![9,7,8,5,6];
        let mut amplifier_chain = AmplifierChain::new(settings, program, AmplifierChainMode::Feedback);
        let result = amplifier_chain.run(0);
        assert_eq!(result, 18216);
    }

}