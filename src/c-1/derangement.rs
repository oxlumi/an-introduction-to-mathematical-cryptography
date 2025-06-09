use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::iter::Iterator;

// Exercise 1.5

// Factorial function
fn factorial(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |acc, x| acc * x)
}

// Derangement D_n = n! * sum_{k=0}^n (-1)^k / k!
fn derangement(n: usize) -> BigUint {
    let mut result = BigUint::zero();
    let n_factorial = factorial(n);
    
    for k in 0..=n {
        let k_factorial = factorial(k);
        if k % 2 == 0 {
            result += &n_factorial / &k_factorial;
        } else {
            result -= &n_factorial / &k_factorial;
        }
    }
    result
}