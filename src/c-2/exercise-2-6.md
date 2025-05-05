# Exercise 2.6 and 2.7

We are given:

* $p = 1373$
* $g = 2$
* $A = g^{e_A} \mod p = 974$
* $e_R = 871$

### Bob’s Public Value $B$

We compute:

$$
B = g^{e_R} \mod p = 2^{871} \mod 1373
$$

#### The binary expansion for that is...

$$
871 = 512 + 256 + 64 + 32 + 4 + 2 + 1 = (1101100111)_2
$$

#### Powers Modulo $p$

| Power     | Computation          | Result |
| --------- | -------------------- | ------ |
| $2^1$     | $2$                  | 2      |
| $2^2$     | $2^2$                | 4      |
| $2^4$     | $(4^2) \mod 1373$    | 16     |
| $2^8$     | $(16^2) \mod 1373$   | 256    |
| $2^{16}$  | $(256^2) \mod 1373$  | 896    |
| $2^{32}$  | $(896^2) \mod 1373$  | 1005   |
| $2^{64}$  | $(1005^2) \mod 1373$ | 377    |
| $2^{128}$ | $(377^2) \mod 1373$  | 870    |
| $2^{256}$ | $(870^2) \mod 1373$  | 209    |
| $2^{512}$ | $(209^2) \mod 1373$  | 1118   |

#### Magic 🪄

$$
2^{871} \equiv 2^{512} \cdot 2^{256} \cdot 2^{64} \cdot 2^{32} \cdot 2^4 \cdot 2^2 \cdot 2^1 \mod 1373
$$

In steps...

1. $1118 \cdot 209 \mod 1373 = 1118 \cdot 209 = 233,662 \mod 1373 = 870$
2. $870 \cdot 377 \mod 1373 = 870 \cdot 377 = 327,990 \mod 1373 = 820$
3. $820 \cdot 1005 \mod 1373 = 820 \cdot 1005 = 824,100 \mod 1373 = 16$
4. $16 \cdot 16 \mod 1373 = 256 \mod 1373 = 256$
5. $256 \cdot 4 \mod 1373 = 1024 \mod 1373 = 1024$
6. $1024 \cdot 2 \mod 1373 = 2048 \mod 1373 = 805$

And finally... our result:

$$
B = 805
$$

Checking this, if we cmpute:

```rust
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
```
We obtain `Bob's public value B: 805`. So yai! we did it ok.

### Shared Secret $S$

$$
S = A^{e_R} \mod p = 974^{871} \mod 1373
$$

#### Powers Modulo $p$

| Power       | Computation         | Result |
| ----------- | ------------------- | ------ |
| $974^1$     | $974$               | 974    |
| $974^2$     | $974^2 \mod 1373$   | 948    |
| $974^4$     | $(948^2) \mod 1373$ | 819    |
| $974^8$     | $(819^2) \mod 1373$ | 405    |
| $974^{16}$  | $(405^2) \mod 1373$ | 500    |
| $974^{32}$  | $(500^2) \mod 1373$ | 744    |
| $974^{64}$  | $(744^2) \mod 1373$ | 449    |
| $974^{128}$ | $(449^2) \mod 1373$ | 951    |
| $974^{256}$ | $(951^2) \mod 1373$ | 468    |
| $974^{512}$ | $(468^2) \mod 1373$ | 1145   |

#### Multiply Relevant Terms

$$
974^{871} \equiv 974^{512} \cdot 974^{256} \cdot 974^{64} \cdot 974^{32} \cdot 974^{16} \cdot 974^8 \cdot 974^1 \mod 1373
$$

In steps...

1. $1145 \cdot 468 \mod 1373 = 535,260 \mod 1373 = 1342$
2. $1342 \cdot 449 \mod 1373 = 602,558 \mod 1373 = 499$
3. $499 \cdot 744 \mod 1373 = 371,256 \mod 1373 = 1134$
4. $1134 \cdot 500 \mod 1373 = 567,000 \mod 1373 = 505$
5. $505 \cdot 405 \mod 1373 = 204,525 \mod 1373 = 352$
6. $352 \cdot 974 \mod 1373 = 342,448 \mod 1373 = 397$


Soo the result is: 

$$
S = 397
$$

And we check it again, using the same function as before we obtaine: `Shared secret s: 397`. Yai!

### Alice’s Secret Exponent $e_A$

We solve:

$$
2^{e_A} \mod 1373 = 974
$$

Using brute-force search : D ... i'm not calculating that by hand, what were you thinking????

```rust
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
```
Sooo out of ✨magic we get:
$$
e_A = 587
$$

| Quantity                      | Result |
| ----------------------------- | ------ |
| Bob’s public value $B$        | 805    |
| Shared secret $S$             | 397    |
| Alice’s secret exponent $e_A$ | 587    |


## 2.7

We are asked: given $g, g^a, g^b, C$, can we decide if:

$$
C \stackrel{?}{=} g^{ab} \mod p
$$

Aaand this is the ✨**Decisional Diffie-Hellman (DDH) problem**✨

### CDH Solves DDH

If we can compute $g^{ab}$ (CDH), we can solve DDH:

1. Compute $g^{ab}$.
2. Check if $C = g^{ab}$.

Thus:

$$
\text{CDH} \implies \text{DDH}
$$

### Is DDH Hard?

| Group Type                        | DDH Hardness     |
| --------------------------------- | ---------------- |
| Prime-order multiplicative groups | Assumed hard     |
| Elliptic curve groups             | Assumed hard     |
| Subgroups with algebraic leaks    | Sometimes easier |

In general, DDH is assumed hard if CDH and discrete log are hard, making it a fundamental cryptographic assumption 😃

### Soooooo...

| Q                    | A                                       |
| --------------------------- | -------------------------------------------- |
| Does solving CDH solve DDH? | Yes, by computing $g^{ab}$ and comparing     |
| Is DDH always hard?         | Generally assumed hard, depends on the group |
| Does solving DDH solve CDH? | No, DDH only distinguishes, not computes     |
