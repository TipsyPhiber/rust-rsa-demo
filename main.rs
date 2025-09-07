/// A minimal implementation of the RSA algorithm to demonstrate the core concepts.
/// This is for educational purposes only and is NOT cryptographically secure.
/// Real-world RSA uses much larger numbers and secure padding schemes.

use std::io::{self, Write}; // Import the standard I/O library

// A modular exponentiation function (base^exp % modulus)
// This is necessary to handle the large numbers in RSA without overflow.
// It uses the "exponentiation by squaring" method for efficiency.
fn power(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        // If exponent is odd, multiply base with result
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        // Exponent must be even now, so we can divide by 2
        exp >>= 1; // exp = exp / 2
        base = (base * base) % modulus;
    }
    result
}

// Encrypt a message using the public key (e, n)
fn encrypt(message: i64, e: i64, n: i64) -> i64 {
    power(message, e, n)
}

// Decrypt a ciphertext using the private key (d, n)
fn decrypt(ciphertext: i64, d: i64, n: i64) -> i64 {
    power(ciphertext, d, n)
}

fn main() {
    println!("--- RSA Algorithm Demonstration ---\n");
    
    // --- KEY GENERATION ---
    println!("[ KEY GENERATION ]");

    // Using small, fixed primes for demonstration.
    let p: i64 = 61;
    let q: i64 = 53;
    let n = p * q;      // n = 3233
    let phi_n = (p - 1) * (q - 1); // phi_n = 3120
    let e: i64 = 17;
    let d: i64 = 2753;   // Pre-calculated for simplicity

    println!("Using primes p={} and q={}", p, q);
    println!("Public Key: (e={}, n={})", e, n);
    println!("Private Key: (d={}, n={})", d, n);
    println!("--> Security relies on the fact that factoring n={} is hard.", n);
    
    // --- INTERACTIVE ENCRYPTION / DECRYPTION ---
    println!("\n[ ENCRYPTION / DECRYPTION CYCLE ]");

    // 1. Prompt the user for a number.
    print!("Enter a number to encrypt (must be less than {}): ", n);
    // We need to 'flush' stdout to ensure the prompt appears before we wait for input.
    io::stdout().flush().unwrap();

    // 2. Read the user's input from the command line.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 3. Parse the input string into a number, with error handling.
    let message: i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nError: Invalid input. Please enter a valid number.");
            return; // Exit the program if input is not a number.
        }
    };

    // 4. Validate that the message is within the allowed range for RSA.
    if message >= n {
        println!("\nError: The message must be a number less than {}.", n);
        return; // Exit if the number is too large.
    }

    println!("\nOriginal Message: {}", message);
    
    // Encrypt the message with the public key
    println!("Encrypting with Public Key...");
    let ciphertext = encrypt(message, e, n);
    println!("  Ciphertext = {}^{} mod {} = {}", message, e, n, ciphertext);
    
    // Decrypt the ciphertext with the private key
    println!("Decrypting with Private Key...");
    let decrypted_message = decrypt(ciphertext, d, n);
    println!("  Decrypted Message = {}^{} mod {} = {}", ciphertext, d, n, decrypted_message);

    println!("\nSuccess! The original message was recovered.");
}
