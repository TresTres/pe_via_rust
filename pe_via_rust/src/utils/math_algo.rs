
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
    fn test_obtain_primes() {
        let mut primes: Vec<u64>;
        primes = sieve_obtain_primes(1);
        assert_eq!(primes, [1]);
        primes = sieve_obtain_primes(10);
        assert_eq!(primes, [1, 2, 3, 5, 7]);
        primes = sieve_obtain_primes(33);
        assert_eq!(primes, [1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    }
}
