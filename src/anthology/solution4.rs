/* Find the largest palindrome made from the product of two 3-digit numbers. */
use crate::utils::strings_algo::is_palindrome;

pub fn process() -> String {

    get_max_palindrome_3()
}

fn get_max_palindrome_3() -> String {
    /*
        The naive implementation is to start with largest 3 digit numbers and keep
        finding products, and return the largest palindromic product.
    */

    let mut product: u64;
    let mut string_product: String;
    let mut max_product: u64 = 0;
    let mut num1: u64 = 999;
    let mut num2: u64 = 999;


    string_product = loop {
        product = num1 * num2;
        string_product = product.to_string();
        if is_palindrome(&string_product) && product > max_product {
            max_product = product;
        }
        num1 -= 1;
        if num1 == 99 {
            if num2 == 100 {
                break max_product.to_string();
            }
            num2 -= 1;
            num1 = 999;
        }

    };
    string_product
}
