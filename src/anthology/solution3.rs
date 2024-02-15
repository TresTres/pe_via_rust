/* Find the largest prime factor of 600851475143 */
pub fn process() -> String {
    let num: u64 = 600851475143;
    let max_factor: u64 = obtain_max_prime_factor(num);
    max_factor.to_string()
}

fn obtain_max_prime_factor(num: u64) -> u64 {
    /*
        Finds the largest prime factor of a given number by iteratively 
        attempting to divide the number
    */
    let mut factor: u64 = 2;
    let mut num_copy: u64 = num;

    while num_copy > factor {
        if num_copy % factor == 0 {
            num_copy = num_copy / factor;
            continue;
        } else {
            factor += 1;
        }
    }
    num_copy
}
 
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_obtain_max_factor() {
        let mut factor;
        factor = obtain_max_prime_factor(1);
        assert_eq!(factor, 1);

        factor = obtain_max_prime_factor(20);
        assert_eq!(factor, 5);

        factor = obtain_max_prime_factor(85);
        assert_eq!(factor, 17);

        factor = obtain_max_prime_factor(23);
        assert_eq!(factor, 23);
    }
}
