/* Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum. */
pub fn process() -> String {

    sum_square_diff(100).to_string()
}


fn sum_square_diff(limit: u64) -> u64 {

    let mut square_of_sums: u64 = 0;
    let mut sum_of_squares: u64 = 0;
    for i in 1..limit+1 {

        square_of_sums += i;
        sum_of_squares += i.pow(2)
    }
    square_of_sums = square_of_sums.pow(2);
    square_of_sums - sum_of_squares
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_square_diff() {
        assert_eq!(sum_square_diff(1), 0);
        assert_eq!(sum_square_diff(5), 170);
    }
}