use crate::solution::{SolutionResult}; 

pub fn run() -> SolutionResult {
    let target: u32 = 1000;
    let total: u32 = get_multotal(target);
    SolutionResult::new(
        1, 
        String::from("Multiples of 3 or 5"),
        total.to_string()
    )
}


fn get_multotal(target: u32) -> u32 {
    let mut multotal: u32 = 0;
    for i in 1..target {
        if i % 3 == 0 || i % 5 == 0 {
            multotal += i;
        }
    }
    multotal
}