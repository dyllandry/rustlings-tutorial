// iterators4.rs

use std::cmp::max;

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // TRY 1
    // return (0..num + 1)
    //     .collect::<Vec<u64>>()
    //     .iter_mut()
    //     .reduce(|accum, item| {
    //         if *item <= 2 {
    //             *accum = max(1, *item);
    //             return accum;
    //         }
    //         *accum *= *item;
    //         return accum;
    //     })
    //     .unwrap()
    //     .clone() as u64;

    // From tutorial
    // return (1..=num).fold(1, |acc, v| acc * v);

    // From tutorial
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
