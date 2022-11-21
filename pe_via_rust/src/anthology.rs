use std::collections::HashMap;

use crate::solution::SolutionInfo;

pub mod solution1;
pub mod solution2;



pub fn load_directory(directory: &mut HashMap<u32, (SolutionInfo, fn() -> String)>) -> () {

    directory.insert(
        1,
        (SolutionInfo { 
            index: 1, problem_name: String::from("Multiples of 3 or 5") }, 
            solution1::process
        )
    );
    directory.insert(
        2,
        (SolutionInfo { 
            index: 2, problem_name: String::from("Even Fibonacci Numbers") }, 
            solution2::process
        )
    );
}