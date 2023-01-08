use crate::utils::math_algo;

/* Find the smallest positive number that is evenly divisible by all of the numbers from 1 to 20. */
pub fn process() -> String {

    let lcm: u64 = lcm_list((1..21).collect());
    lcm.to_string()
}

fn lcm_list(vec: Vec<u64>) -> u64 {
    /*
        Compute the LCM of a collection of integers
    */
    return vec.into_iter().reduce(|acc, el| math_algo::get_lcm(acc, el)).unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[test]
    #[should_panic]
    fn test_lcm_list_zeros() { 
        let list: Vec<u64> = Vec::from([0, 0, 0]);
        lcm_list(list);
    }

    #[rstest]
    #[case(Vec::from([0, 1, 2, 3]), 0)]
    #[case(Vec::from([1, 2, 3]), 6)]
    #[case(Vec::from([3, 9, 27]), 27)]
    #[case(Vec::from([1, 2, 3, 4, 5]), 60)]
    fn test_lcm_list(#[case] list_input: Vec<u64>, #[case] expected_lcm: u64) {
        assert_eq!(lcm_list(list_input), expected_lcm);
    }
}