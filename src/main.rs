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

// all cases
fn main() {
    let n = 26;
    let total_permutations = factorial(n);
    let d_n = derangement(n);
    let at_least_one_fixed = &total_permutations - &d_n;
    let d_n_minus_1 = derangement(n - 1);
    let exactly_one_fixed = BigUint::from(n) * d_n_minus_1;
    let at_least_two_fixed = &at_least_one_fixed - &exactly_one_fixed;

    println!("Total permutations (26!): {}", total_permutations);
    println!("Derangements (no letters fixed): {}", d_n);
    println!("At least one letter fixed: {}", at_least_one_fixed);
    println!("Exactly one letter fixed: {}", exactly_one_fixed);
    println!("At least two letters fixed: {}", at_least_two_fixed);
}
