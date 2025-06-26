// ElGamal 
  
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::Rng;

fn main(){
    // We choose a large prime p and an element g modulo p of a large order
    let p = BigUint::from_u32(23).unwrap(); 
    let g = BigUint::from_u32(5).unwrap(); // We create the value from 5
    
    // Alicia chooses a private key 1 <= a <= p-1 
    let a = BigUint::from_u32(6).unwrap();
    
    // Computes A = g^a (p)
    let A = g.modpow(&a, &p);
    
    // Sends A to Bob
    println!("Alice's public key: {}", A);
    
    // Roberto takes the plain text m
    let m = "Wonderland!";
    // Convert m to a binary string
    let m_binary = m.as_bytes().to_vec();
    
    // Choose a random ephemeral key k
    let k = BigUint::from_u32(15).unwrap();
    
    // We use Alicia public key A to compute c_1 = g^k (mod p) and c_2 = m * A^k (p) 
    let c1 = g.modpow(&k, &p);
    let a_k = A.modpow(&k, &p);
    let m_biguint = BigUint::from_bytes_be(&m_binary);
    let c2 = (m_biguint * a_k) % &p;
    
    // We send the tuple (c_1, c_2) to Alicia
    println!("Roberto's ciphertext: ({}, {})", c1, c2);
    
    // Alicia now needs to compute (C_1^a)^-1 * C_2 (mod p) and this should be equal to m
    let a_inv = A.modpow(&(p - BigUint::from(1u32)), &p);
    let m_decrypted = c2.modpow(&a_inv, &p);
    
    // Convert the decrypted message back to a string
    let m_decrypted_str = String::from_utf8(m_decrypted.to_bytes_be()).unwrap();
    
    println!("Decrypted message: {}", m_decrypted_str);
}
