
pub fn is_palindrome(str: &String) -> bool {
    /* Check if a string is a palindrome using two pointers */
    //Optimize for trivial case
    let count: usize = str.chars().count();

    if count <= 1 {
        return true;
    }

    let mut result: bool = true;
    let mut front_index: usize;
    let mut back_index: usize;

    let mut c1: char;
    let mut c2: char;


    if count % 2 == 0 {
        front_index = count / 2 - 1;
        back_index = count / 2;
    } else {
        front_index = (count - 1) / 2;
        back_index = front_index;
    }

    loop {
        c1 = str.chars().nth(front_index).unwrap();
        c2 = str.chars().nth(back_index).unwrap();

        if c1 != c2 {
            result = false;
            break;
        }    
        if front_index == 0 {
            break;
        }

        back_index += 1;
        front_index -= 1;
    } 
    result
} 


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_palindrome() {
        let mut str: String;
        str = String::from("x");
        assert_eq!(is_palindrome(&str), true);
        str = String::from("Hello");
        assert_eq!(is_palindrome(&str), false);
        str = String::from("noon");
        assert_eq!(is_palindrome(&str), true);
        str = String::from("foo💰oof");
        assert_eq!(is_palindrome(&str), true);
    }

}
