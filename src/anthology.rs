use crate::solution::{SolutionResult, SolutionInfo};
use std::collections::HashMap;
pub mod solution1;
pub mod solution2;
pub mod solution3;
pub mod solution4;
pub mod solution5;
pub mod solution6;

pub struct Directory {
    directory: HashMap<u32, (SolutionInfo, fn() -> String)>,
    index: Vec<u32>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            directory: HashMap::new(),
            index: Vec::new(),
        }
    }

    pub fn insert(&mut self, index: u32, info: SolutionInfo, process: fn() -> String) -> () {
        self.directory.insert(index, (info, process));
        self.index.push(index);
    }

    pub fn get(&self, index: u32) -> Option<&(SolutionInfo, fn() -> String)> {
        self.directory.get(&index)
    }

    pub fn run_solution(&self, problem_number: u32) -> SolutionResult {
        if let Some((info, process)) = self.get(problem_number) {
            return SolutionResult::new(info.index, info.problem_name.clone(), process());
        }
        panic!("Could not find problem number: {}", problem_number);
    }


    pub fn list(&self) -> () {
        for index in self.index.iter() {
            if let Some((info, _)) = self.get(*index) {
                println!("Problem {}: {}", info.index, info.problem_name);
            } else {
                panic!("Could not find problem number: {}", index);
            }
        }
    }
}

pub fn load_directory() -> Directory {
    let mut directory = Directory::new();
    directory.insert(
        1,
        SolutionInfo {
            index: 1,
            problem_name: String::from("Multiples of 3 or 5"),
        },
        solution1::process,
    );
    directory.insert(
        2,
        SolutionInfo {
            index: 2,
            problem_name: String::from("Even Fibonacci Numbers"),
        },
        solution2::process,
    );
    directory.insert(
        3,
        SolutionInfo {
            index: 3,
            problem_name: String::from("Largest Prime Factor"),
        },
        solution3::process,
    );
    directory.insert(
        4,
        SolutionInfo {
            index: 4,
            problem_name: String::from("Largest Palindrome Product"),
        },
        solution4::process,
    );
    directory.insert(
        5,
        SolutionInfo {
            index: 5,
            problem_name: String::from("Smallest Multiple"),
        },
        solution5::process,
    );
    directory.insert(
        6,
        SolutionInfo {
            index: 6,
            problem_name: String::from("Sum Square Difference"),
        },
        solution6::process,
    );
    directory
}
