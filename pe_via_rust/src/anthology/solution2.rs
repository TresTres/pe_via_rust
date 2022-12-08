/* Find the sum of the even valued terms in the Fibonacci sequence that don't exceed 4000000 */
pub fn process() -> String {
    /*
        Use a closure to capture the total
        and increase it when the term in the sequence is even.
    */
    let limit: u32 = 4000000;
    let mut total: u32 = 0;
    {
        let collect_logic = |x: u32| -> () {
            if x % 2 == 0 {
                total += x;
            }
        };
        fib_collect(limit, collect_logic);
    }
    total.to_string()
}

fn fib_collect<F>(limit: u32, mut collect: F) -> ()
where
    F: FnMut(u32) -> (),
{
    /*
        Let a closure (described by FnMut trait) subscribe to
        Fibonacci sequence generation up to a limit.
    */
    let mut lag_term: u32 = 1;
    let mut term: u32 = 2;
    collect(lag_term);
    while term < limit {
        collect(term);
        let tmp: u32 = lag_term;
        lag_term = term;
        term = lag_term + tmp;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fib_collect() {
        let mut total: u32 = 0;
        let collect = |x: u32| -> () {
            total += x;
        };
        fib_collect(10, collect);
        assert_eq!(total, 19);
    }
}
