use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::Instant;

// Modular exponentiation: (base^exp) mod modulus
fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base.clone() % modulus;
    let mut exp = exp.clone();

    while !exp.is_zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp /= 2u32;
    }
    result
}

// Brute-force discrete log (slow for big exponents!)
fn discrete_log_brute(g: &BigUint, target: &BigUint, modulus: &BigUint) -> Option<BigUint> {
    let mut power = BigUint::one();
    let mut exponent = BigUint::zero();

    while &power != target {
        exponent += 1u32;
        power = (&power * g) % modulus;

        if exponent > *modulus {
            return None; // not found
        }
    }
    Some(exponent)
}

fn main() {
    let p = BigUint::from(1373u32);
    let g = BigUint::from(2u32);
    let a_pub = BigUint::from(974u32);
    let b_secret = BigUint::from(871u32);

    let start = Instant::now();

    // Compute Bob's public value B
    let b_pub = mod_pow(&g, &b_secret, &p);
    println!("Bob's public value B: {}", b_pub);

    // Compute shared secret s = A^b mod p
    let shared_secret = mod_pow(&a_pub, &b_secret, &p);
    println!("Shared secret s: {}", shared_secret);

    // Compute Alice's secret exponent a (brute-force)
    println!("Computing Alice's secret exponent (this may take time)... wait for it... waaaaaaait for it....");
    match discrete_log_brute(&g, &a_pub, &p) {
        Some(a_secret) => println!("Alice's secret exponent a: {}", a_secret),
        None => println!("Could not find Alice's secret exponent"),
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
