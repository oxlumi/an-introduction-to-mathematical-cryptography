/* 
The euclides algorithm says that to calculate gcd(a,b), we can compute:
a / b = b * k_0 + r_0
b / r_0 = r_0 * k_1 + r_1
And so on, until r_n = 0 and our k_n will be the gcd(a,b)
*/

fn euclides(mut a: u64, mut b: u64) -> u64 {
    // we continue until b becomes zero.
    while b != 0 {
        // remainder of a divided by b.
        let r = a % b;
        // Assign the value of b to a.
        a = b;
        // Assign the remainder to b.
        b = r;
    }
    // When b is zero, a contains the gcd of the original a and b.
    a
}
