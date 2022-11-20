use std::fmt;

struct SolutionInfo {
    index: u32,
    problem_name: String,
}

pub struct SolutionResult {
    info: SolutionInfo,
    ans: String,
}

impl SolutionResult {
    pub fn new(index: u32, problem_name: String, ans: String) -> SolutionResult {
        SolutionResult {
            info: SolutionInfo {
                index, 
                problem_name
            },
            ans
        }
    }
}

impl fmt::Display for SolutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Solution for problem {} - {}: {}",
            self.info.index, self.info.problem_name, self.ans
        )
    }
}


