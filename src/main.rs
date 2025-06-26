// ElGamal 
  
use num_bigint::BigUint;
use num_traits::FromPrimitive;

fn main(){
    
    let message = "Encrypted Wonderland";
    let message_bytes = message.as_bytes().to_vec();
    let message_biguint = BigUint::from_bytes_be(&message_bytes); // i want to check how large is m
    println!("Message '{}' as number: {}", message, message_biguint);
    
   
    let p_str = "6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151";   // (Mersenne prime 2^521 - 1)
    let p = p_str.parse::<BigUint>().unwrap();
    let g = BigUint::from_u32(2).unwrap(); // Generator
    
    println!("Prime p: {}", p);
    println!("Is the prime p larger than the message?: {}", message_biguint < p);
    
    // Alicia chooses a private key 
    let a = BigUint::from_u32(12345).unwrap();
    
    // Computes A = g^a (mod p) ---> Alicia's public key
    let A = g.modpow(&a, &p);
    
    println!("Alicia's public key: {}", A);
    
    // Roberto takes the plain text m
    let m_biguint = message_biguint;
    println!("Original message as number: {}", m_biguint);
    
    // Choose a random ephemeral key k
    let k = BigUint::from_u32(54321).unwrap();
    
    // Roberto uses Alicia's public key A to compute:
    // c_1 = g^k (mod p) and c_2 = m * A^k (mod p) 
    let c1 = g.modpow(&k, &p);
    let a_k = A.modpow(&k, &p);
    let c2 = (&m_biguint * &a_k) % &p;
    
    // Send the tuple (c_1, c_2) to Alicia
    println!("Roberto's ciphertext:");
    println!("c1: {}", c1);
    println!("c2: {}", c2);
    
    // Alicia decrypts: (c_1^a)^(-1) * c_2 (mod p)
    let c1_a = c1.modpow(&a, &p);  // c_1^a (mod p)
    
    // Find modular inverse using Fermat's little theorem: x^(-1) = x^(p-2) (mod p) < 3 
    let c1_a_inv = c1_a.modpow(&(&p - BigUint::from_u32(2).unwrap()), &p);
    
    // m = c1_a_inv * c2 (mod p)
    let m_decrypted = (&c1_a_inv * &c2) % &p;
    
    // Convert the decrypted message back to a string
    let m_decrypted_bytes = m_decrypted.to_bytes_be();
    
    match String::from_utf8(m_decrypted_bytes) {
        Ok(m_decrypted_str) => {
            println!("Decrypted message: '{}'", m_decrypted_str);
            println!("Decryption successful: {}", message == m_decrypted_str);
        }
        Err(_) => {
            println!("Decrypted number: {}", m_decrypted);
            println!("Original number: {}", m_biguint);
            println!("Decryption successful: {}", m_biguint == m_decrypted);
        }
    }
}
