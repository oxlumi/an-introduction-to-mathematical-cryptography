# Exercise 2.6

### Given

* Prime: \$p = 1373\$
* Generator: \$g = 2\$
* Alice's public value: \$A = 974\$
* Bob's secret exponent: \$b = 871\$

We're going to:
- Compute Bob's public value \$B\$
- Compute the shared secret \$s\$
- Find Alice's secret exponent \$a\$

## Bob's Public Value \$B\$

We wave our math wand to compute:

$$
B = g^b \mod p = 2^{871} \mod 1373
$$

✅ **Rust output:**

```
Bob's public value B: 805
```

Thus:

$$
B = 805
$$

🎉 **Bob sends this magical number to Alice.**

---

### ✨ Step 2: Compute the Shared Secret \$s\$

The shared secret (what both Alice and Bob will magically agree on) is:

$$
s = A^b \mod p = 974^{871} \mod 1373
$$

✅ **Rust output:**

```
Shared secret s: 397
```

Thus:

$$
s = 397
$$

🎉 **Both sides now share this secret value. It's like synchronized magic!**

---

### ✨ Step 3: Find Alice's Secret Exponent \$a\$

We need to solve the mysterious equation:

$$
2^a \mod 1373 = 974
$$

✅ **Rust brute-force search output:**

```
Alice's secret exponent a: 587
```

Thus:

$$
a = 587
$$

🎉 **Alice's secret revealed!** (Don’t tell anyone, though.)

---

## 🦀 Rust Code (Our Spellbook)

```rust
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::Instant;

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

    let b_pub = mod_pow(&g, &b_secret, &p);
    println!("Bob's public value B: {}", b_pub);

    let shared_secret = mod_pow(&a_pub, &b_secret, &p);
    println!("Shared secret s: {}", shared_secret);

    println!("Computing Alice's secret exponent (this may take time)... wait for it... waaaaaaait for it....");
    match discrete_log_brute(&g, &a_pub, &p) {
        Some(a_secret) => println!("Alice's secret exponent a: {}", a_secret),
        None => println!("Could not find Alice's secret exponent"),
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
```

✅ **Cargo Run Output:**

```
Bob's public value B: 805
Shared secret s: 397
Computing Alice's secret exponent (this may take time)... wait for it... waaaaaaait for it....
Alice's secret exponent a: 587
Execution time: 775.75µs
```

---

### 📝 Final Magic Summary

| Quantity                      | Result |
| ----------------------------- | ------ |
| Bob's public \$B\$            | 805    |
| Shared secret \$s\$           | 397    |
| Alice's secret exponent \$a\$ | 587    |

🎉✨ **Congratulations! You've completed Exercise 1 — with math, code, and crypto magic.**

---

## 🌟 Exercise 2 — Diffie-Hellman Decision Problem

### Recap

Given:

* \$p\$, prime
* \$g\$, generator
* \$A = g^a \mod p\$
* \$B = g^b \mod p\$
* \$C\$, some candidate value

We want to decide:

$$
C \stackrel{?}{=} g^{ab} \mod p
$$

This is called the **Decisional Diffie-Hellman (DDH) problem**.

---

### ✨ Step 1: Understand the Difference

| Problem                            | Task                                    |
| ---------------------------------- | --------------------------------------- |
| Computational Diffie-Hellman (CDH) | Compute \$g^{ab}\$ from \$g^a, g^b\$    |
| Decisional Diffie-Hellman (DDH)    | Decide if given \$C\$ equals \$g^{ab}\$ |

✅ **Key idea:** If you can solve CDH, you can trivially solve DDH by computing \$g^{ab}\$ and comparing.

---

### ✨ Step 2: Formal Proof

✅ **Proof that CDH ⇒ DDH:**

Assume we have an algorithm \$\text{CDH}(g, g^a, g^b)\$ that outputs \$g^{ab}\$.

Given \$(g, g^a, g^b, C)\$:

1. Compute \$\hat{C} = \text{CDH}(g, g^a, g^b) = g^{ab}\$.
2. Check if \$\hat{C} = C\$.

✅ Done! If CDH is solvable, DDH is solved too.

---

### ✨ Step 3: Is DDH Hard or Easy?

| Group Type                 | DDH Hardness  |
| -------------------------- | ------------- |
| General prime-order groups | Believed hard |
| Elliptic curve groups      | Believed hard |
| Some subgroups with leaks  | May be easier |

✅ **Conclusion:** In most cryptographic groups, DDH is assumed hard — no efficient test is known to distinguish \$g^{ab}\$ from random without solving CDH.

---

### 🦀 Rust Sketch for DDH (Optional)

```rust
fn decisional_dh(g: &BigUint, a_pub: &BigUint, b_pub: &BigUint, c_candidate: &BigUint, p: &BigUint) -> bool {
    let a = discrete_log_brute(g, a_pub, p).expect("Could not find a");
    let b = discrete_log_brute(g, b_pub, p).expect("Could not find b");
    let computed = mod_pow(g, &(a * b), p);
    &computed == c_candidate
}
```
Brute force hehe.