use std::cmp;
use std::collections::BTreeMap;

use super::search_algo;

pub fn get_gcd(a: u64, b: u64) -> u64 {
    /*
        Use Euclid's algorithm to get greatest common divisor.
    */
    if a == 0 && b == 0 {
        panic!("One of the inputs to gcd must be non-zero");
    }
    let mut big: u64 = cmp::max(a, b);
    let mut small: u64 = cmp::min(a, b);
    if small == 1 {
        return 1;
    }
    if small == 0 {
        return big;
    }
    let mut rem: u64;

    while small != 0 {

        rem = big % small;
        big = small;
        small = rem;
    }

    return big;
}

pub fn get_lcm(a: u64, b: u64) -> u64 {
    /*
        Compute the lcm from the gcd of two numbers.
    */
    return a * b / get_gcd(a, b);
}


pub fn nth_prime(n: u64) -> u64 {
    /*
        Find the nth prime number.
    */

    // Maintain primes and their positions
    let mut primes: Vec<u64> = vec![2];
    let mut prime_positions: BTreeMap<u64, u32> = BTreeMap::new();
    prime_positions.insert(2, 0);

    let mut candidate: u64 = 2;

    while primes.len() < n as usize {
        
        /*
            Search for the next prime number iteratively.
            Start by checking the number after the last known prime number. 
            Then test for primality by checking if the number is divisible by known primes.
            Optimize by checking prime numbers that are less than or equal to the square root of the candidate.
        */
        candidate = primes.last().unwrap() + 1;
        loop {
            let mut factor_found: bool = false;
            let square_root: u64 = (candidate as f64).sqrt().ceil() as u64;
            let starting_position: u32 = search_algo::find_nearest_element_less_than::<u64, u32>(&prime_positions, &square_root).1; 
            for prime in primes[0..=starting_position as usize].iter().rev() {
                if candidate % prime == 0 {
                    factor_found = true;
                    break;
                }
            }
            if factor_found {
                candidate += 1;
                continue;
            }
            break;
        }
        primes.push(candidate);
        prime_positions.insert(candidate, primes.len() as u32 - 1);
    }
    candidate
}





#[allow(dead_code)]
fn sieve_obtain_primes(num: u64) -> Vec<u64> {
    /*
        Obtain all the prime numbers up to num
        using the sieve of Erasthones: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

        At values of num reaching 
        10^7:  4s
        10^8 : 45s
        10^9 : 8 minutes

        For values above 10^8 please choose a different algorithm.
    */

    let mut start_check: u64;
    let mut primes: Vec<u64> = Vec::new();
    // optimize by limiting factorization up to root of number
    let factor_limit: u64 = ((num as f64).sqrt().ceil()) as u64;
    let mut candidacy_map: Vec<bool> = vec![true; num as usize];

    for i in 2..factor_limit {
        // optimize by skipping sieves that are known composites
        if candidacy_map[(i -1) as usize] == false {
            continue;
        }
        // optimize by starting sieve with the square of the sieve
        start_check = i.pow(2);
        for j in ((start_check)..=num).step_by(i as usize) {
            candidacy_map[(j - 1) as usize] = false;
        }
    }

    // collection of primes
    for b in candidacy_map.iter().enumerate() {
        if *b.1 == true {
            primes.push((b.0 + 1) as u64);
        }
    }
    primes
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn test_gcd_zero() {
        get_gcd(0, 0);
    }

    #[test]
    fn test_get_gcd() {
        let mut gcd: u64;
        gcd = get_gcd(1, 9);
        assert_eq!(gcd, 1);
        gcd = get_gcd(5, 5);
        assert_eq!(gcd, 5);
        gcd = get_gcd(0, 8);
        assert_eq!(gcd, 8);
        gcd = get_gcd(5, 2);
        assert_eq!(gcd, 1);
        gcd = get_gcd(5, 10);
        assert_eq!(gcd, 5);
        gcd = get_gcd(21, 49);
        assert_eq!(gcd, 7);
    }

    #[test]
    fn test_get_lcm() {
        let mut lcm: u64;
        lcm = get_lcm(7, 1);
        assert_eq!(lcm, 7);
        lcm = get_lcm(0, 8);
        assert_eq!(lcm, 0);
        lcm = get_lcm(3, 9);
        assert_eq!(lcm, 9);
        lcm = get_lcm(3, 5);
        assert_eq!(lcm, 15);
    }

    #[test]
    fn test_obtain_primes() {
        let mut primes: Vec<u64>;
        primes = sieve_obtain_primes(1);
        assert_eq!(primes, [1]);
        primes = sieve_obtain_primes(10);
        assert_eq!(primes, [1, 2, 3, 5, 7]);
        primes = sieve_obtain_primes(33);
        assert_eq!(primes, [1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    }


    #[test]
    fn test_nth_prime() {
        let mut prime: u64;
        prime = nth_prime(1);
        assert_eq!(prime, 2);
        prime = nth_prime(2);
        assert_eq!(prime, 3);
        prime = nth_prime(3);
        assert_eq!(prime, 5);
        prime = nth_prime(4);
        assert_eq!(prime, 7);
        prime = nth_prime(5);
        assert_eq!(prime, 11);
        //...and so on
        prime = nth_prime(10);
        assert_eq!(prime, 29);
        prime = nth_prime(100);
        assert_eq!(prime, 541);
    }
}
