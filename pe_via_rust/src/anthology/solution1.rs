/* Find the sum of all multiples of 3 or 5 below 1000 */
pub fn process() -> String {
    let target: u32 = 1000;
    let total: u32 = get_multotal(target);
    total.to_string()
}

fn get_multotal(target: u32) -> u32 {
    /*
        Get total of numbers that are divisble by 3 or 5
    */
    let mut multotal: u32 = 0;
    for i in 1..target {
        if i % 3 == 0 || i % 5 == 0 {
            multotal += i;
        }
    }
    multotal
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multotal() {
        assert_eq!(get_multotal(0), 0);
        assert_eq!(get_multotal(2), 0);
        assert_eq!(get_multotal(15), 45);
    }
}