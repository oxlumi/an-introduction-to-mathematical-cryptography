
/* 
The recursive tree for fast_power_mod(2, 13, 17) would be:

2^13 
├── 2^6 * 2 
│   ├── 2^3 * 2^3
│   │   ├── 2^1 * 2^1 * 2
│   │   │   ├── 2^0 * 2^0 * 2 = 1 * 1 * 2 = 2
│   │   │   └── Returns: 2
│   │   └── Returns: (2 * 2) % 17 * 2 = 8
│   └── Returns: (8 * 8) % 17 = 13
└── Returns: (13 * 13) % 17 * 2 % 17 = 15

Each level shows:
- How the exponent is split (exp/2)
- The intermediate calculations with modulus
- The final return value for that recursive call

*/

fn fast_power_mod(base: u64, exp: u64, modulus: u64) -> u64 {
    if exp == 0 { // Base case
        return 1;
    }
    let half = fast_power_mod(base, exp / 2, modulus); // Recursive call
    let half_squared = (half * half) % modulus;
    if exp % 2 == 0 { // If exp is even
        half_squared
    } else { // If exp is odd
        (base * half_squared) % modulus
    }
}
