use crate::solution::SolutionResult;

pub fn run() -> SolutionResult {
    let limit: u32 = 4000000;
    // closure where we capture the total and 
    // increase it when the term is even
    let mut total: u32 = 0;
    let collect_logic = | x: u32 | -> () {
        if x % 2 == 0 {
            total += x;
        }
    };
    fib_collect(limit, collect_logic);
    SolutionResult::new(2, String::from("Even Fibonacci Numbers"), total.to_string())
}

fn fib_collect<F>(limit: u32, mut collect: F) -> () where F: FnMut(u32) -> (){
    // let a closure subscribe to fibonacci number generation
    // to operate on terms
    let mut lag_term: u32 = 1;
    let mut term: u32 = 2;
    while term < limit {
        collect(term);
        let tmp: u32 = lag_term;
        lag_term = term;
        term = lag_term + tmp;
    }
}