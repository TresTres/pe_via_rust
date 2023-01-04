use std::collections::HashMap;

mod anthology;
mod solution;
mod utils;


fn main() {
    /*
        Specify a problem number to solve.
        Load a directory containing solution descriptions and algorithms.
        Obtain the solution by keying on the problem number and print the result.
    */

    let problem_number: u32 = 4;
    let mut directory: HashMap<u32, (solution::SolutionInfo, fn() -> String)> = HashMap::new();
    anthology::load_directory(&mut directory);
    if let Some((info, process)) = directory.get(&problem_number) {
        println!("{}", run_solution(info, process));    
    }
}


fn run_solution(info: &solution::SolutionInfo, process: &fn() -> String) -> solution::SolutionResult {
    /*
        return a SolutionResult by running the process function
    */
    solution::SolutionResult::new(
        info.index,
        info.problem_name.clone(),
        process()
    )
}
