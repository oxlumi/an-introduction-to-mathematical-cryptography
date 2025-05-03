# An Introduction to Mathematical Cryptography (Hoffstein, Pipher, Silverman) - Chapter 1 Notes

**Main property:** no two plaintext letters go to the same ciphertext (injective).  
> The level of security of a system can only get worse.

### 1.2 Divisibility and GCD

**Definition:** Let $a, b \in \mathbb{Z}$ and $ b \neq 0 $, we say that $ b \mid a $ only if $ \exists \space c \in \mathbb{Z} :$ $a = b \cdot c$

Let $ a, b, c \in \mathbb{Z} $, we see that:
1. if $ a \mid b $ and $ b \mid c $ ⇒ $ a \mid c $
2. if $ a \mid b $ and $ a \mid -b $ ⇒ $ a \mid b $
3. if $ a \mid b $ and $ a \mid c $ ⇒ $ a \mid (b + c) $ ∧ $ a \mid (b - c) $

#### Proof

$ a, b, c \in \mathbb{Z} $

- $ a \mid b $ ⇒ $ b = a \cdot k $  (1)  
- $ b \mid c $ ⇒ $ c = b \cdot n = (a \cdot k) \cdot n $  (2)

Now if $ a \mid c $ ⇒ $ c = a \cdot m $ and as $ m \in \mathbb{Z} $, we can write it as $ k \cdot n $, which leads us to (2) again.

Now if $ a \mid b $ and $ b \mid a $ ⇒ $ a = \pm b $.

- $ a \mid b $ ⇒ $ b = a \cdot k $
- $ b \mid a $ ⇒ $ a = b \cdot n $

Thus $a (1 - k \cdot n) = 0$ and if $ a \neq 0 $ ⇒ $ k \cdot n = 1 $, and the only solutions are $ k = n = 1 $ or $ k = n = -1 $.

Therefore:
- If $ m = 1 $, $ b = a $
- If $ m = -1 $, $ b = -a $

Lastly, if $ a \mid b $ ∧ $ a \mid c $ ⇒ $ a \mid (b + c) $.

- $ a \mid b $ ⇒ $ b = a \cdot k $
- $ a \mid c $ ⇒ $ c = a \cdot h $

$(b + c) = a \cdot k + a \cdot h = a (k + h)$

**Definition**: A common divisor of two integers $ a $ and $ b $ is a positive integer $ d $ that divides both of them. The greatest common divisor is the largest positive integer $ d $ such that $ d \mid a $ ∧ $ d \mid b $. It’s denoted: $\gcd(a, b) \text{ or } (a, b)$

### Division Algorithm

$ a, b \in \mathbb{Z}, a = b \cdot q + r $ with $ 0 \leq r < b $.

We also have the Euclidean algorithm that helps us to find the $ \gcd(a, b) $ of two numbers.

The key concept is that if $ a = bq + r $, then: $\gcd(a, b) = \gcd(b, r)$ because any divisor of $ a $ or $ b $ should also divide $ r $.

**Steps:**
1. Let $ r_0 = a $ and $ r_1 = b $.
2. Set $ i = 1 $.
3. Divide $ r_{i-1} $ by $ r_i $ to get quotient $ q_i $ and remainder $ r_{i+1} $: $r_{i-1} = q_i \cdot r_i + r_{i+1}$ with $ 0 \leq r_{i+1} < r_i $.

4. If $ r_{i+1} = 0 $ ⇒ $ r_i $ is $ \gcd(a, b) $ and the algorithm terminates.
5. Otherwise, $ r_{i+1} \neq 0 $, so set $ i = i + 1 $ and go to step (3).

Executed maximum $ 2 \cdot \log_2(b) + 1 $ times.

### 1.3 Modular Arithmetic

Let $ m \geq 1 $ be an integer. We say that $ a, b \in \mathbb{Z} $ are "congruent" modulo $ m $ if their difference $ a - b $ is divisible by $ m $: $a \equiv b \pmod{m}$.

- If $ a \equiv 0 \pmod{m} $ ⇒ $ a $ is a multiple of $ m $.

Some key properties are:
- (a) If $ a_1 \equiv a_2 \pmod{m} $ and $ b_1 \equiv b_2 \pmod{m} $, then:
  \[ a_1 \pm b_1 \equiv a_2 \pm b_2 \]
  and
  \[ a_1 b_1 \equiv a_2 b_2 \pmod{m} \]

- (b) Let $ a \in \mathbb{Z} $, $ a \cdot b \equiv 1 \pmod{m} $ for some integer $ b $ if and only if $ \gcd(a, m) = 1 $.

If such integer exists, we say that it is the multiplicative inverse of $ a $ modulo $ m $.

If divided by $ m $ has quotient $ q $ and remainder $ r $, it can be written as: $a = m \cdot q + r$

This shows that: $a \equiv r \pmod{m}$ for some integer $ r $ between 0 and $ m - 1 $.

So, if we want to work with integers modulo $ m $, it is enough to use the integers $ 0 \leq r < m $.

Based on that, we can write:
$ \mathbb{Z} / m\mathbb{Z} = \{ 0, 1, 2, \dots, m-1 \}$ and call $ \mathbb{Z} / m\mathbb{Z} $ the ring of integers modulo $ m $.

Whenever we perform addition or multiplication in $ \mathbb{Z} / m\mathbb{Z} $, we always divide the result by $ m $ and take the remainder in order to obtain an element in $ \mathbb{Z} / m\mathbb{Z} $.

### Units and Euler's Phi Function

$ a $ has an inverse modulo $ m $ if and only if $ \gcd(a, m) = 1 $.

Numbers that have inverses are called **units**:
$$ (\mathbb{Z} / m\mathbb{Z})^* = \{ a \in \mathbb{Z} / m\mathbb{Z} \mid \gcd(a, m) = 1 \} $$

It is called the **group of units** modulo $ m $.

Usually, it is important to know how many elements are in the unit group modulo $ m $.

- $ \varphi $ = Euler phi function: $\varphi(m) = \# (\mathbb{Z} / m\mathbb{Z})^* = \# \{ 0 < a < m \mid \gcd(a, m) = 1 \} $

### Modular Arithmetic and Shift Ciphers

A shift cipher with shift $ k $ takes a plaintext letter corresponding to the number $ p $ and assigns it to the ciphertext letter corresponding to the number: $p + k \pmod{26}$

Then our encryption key would be:
$(\text{Ciphertext letter}) \equiv (\text{Plaintext letter}) + (5k) \pmod{26}$

And decryption:
$(\text{Plaintext letter}) \equiv (\text{Ciphertext letter}) - (5k) \pmod{26}$

Or:
$C \equiv p + k \pmod{26}$
and
$p \equiv C - k \pmod{26}$

### Fast Powering Algorithm

TL;DR It’s an efficient way to compute $ b^e $ modulo some number $ m $. It uses **divide and conquer** to reduce time to $ O(\log e) $.

The core idea is:
- If $ e $ is even:
  $b^e = (b^{e/2})^2$
- If $ e $ is odd:
  $b^e = b \cdot b^{e-1}$

We can recursively break the exponentiation into smaller problems, squaring intermediate results rather than multiplying many times.

The recursive formula is:
$$
\text{FastPower}(b, e) =
\begin{cases}
1, & \text{if } e = 0 \\
\text{FastPower}(b, e/2)^2, & \text{if } e \text{ is even} \\
b \cdot \text{FastPower}(b, e-1), & \text{if } e \text{ is odd}
\end{cases}
$$

> See `fast-powering-algorithm.rs` for the Rust implementation.