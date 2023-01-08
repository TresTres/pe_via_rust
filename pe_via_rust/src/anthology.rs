use std::collections::HashMap;
use crate::solution::SolutionInfo;
pub mod solution1;
pub mod solution2;
pub mod solution3;
pub mod solution4;
pub mod solution5;


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
    directory.insert(
        3,
        (SolutionInfo { 
            index: 3, problem_name: String::from("Largest Prime Factor") },
            solution3::process
        )
    );
    directory.insert(
        4,
        (SolutionInfo { 
            index: 4, problem_name: String::from("Largest Palindrome Product") },
            solution4::process
        )
    );
    directory.insert(
        5,
        (SolutionInfo {
            index: 5, problem_name: String::from("Smallest Multiple") },
            solution5::process
        )
    );
}