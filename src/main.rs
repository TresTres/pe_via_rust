use std::env;

mod anthology;
mod solution;
mod utils;

fn main() {
    /*
        Specify a problem number to solve.
        Load a directory containing solution descriptions and algorithms.
        Obtain the solution by keying on the problem number and print the result.
    */
    let args: Vec<String> = env::args().collect();
    let directory: anthology::Directory = anthology::load_directory();

    // check arguments
    if args.len() < 2 {
        panic!("No arguments provided. Please see cargo run help");
    }
    let argument: &str = args[1].trim();
    let problem_number: Option<u32> = match argument.parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    // run solution or handle sub command
    if let Some(problem_number) = problem_number {
        return println!("{}", directory.run_solution(problem_number));
    } 
    match argument {
        "help" => {
            println!("Run a solution: cargo run <problem_number>");
            println!("Obtain available problem numbers: cargo run list");
        }
        "list" => {
            return directory.list();
        }
        _ => {
            println!("Invalid argument: {}", argument);
            println!("Please see cargo run help");
        }
    }
}
