/* Find the nth prime number.  In this case n = 10 001 */
use crate::utils::math_algo::nth_prime;

pub fn process() -> String {
    nth_prime(10_001).to_string()
}
