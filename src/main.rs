use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::Num;
mod authenticator;
mod client;
use crate::authenticator::Authenticator;
use crate::client::Client;

pub fn generate_large_prime(bits: usize) -> BigUint {
    let prime = Generator::new_prime(bits).to_string();
    BigUint::from_str_radix(&prime, 10).expect("Prime generation failed")
}

pub fn authentication_protocol(iterations: usize) -> bool {
    let prime1 = generate_large_prime(256);
    let prime2 = generate_large_prime(256);

    let mut secrets: Vec<BigUint> = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        secrets.push(generate_large_prime(256));
    }

    let client = Client::initialize(prime1, prime2, secrets);
    let auth = Authenticator::initialize(client.modulus.clone(), client.public_keys.clone());

    for step in 0..iterations {
        println!("Authentication Step: {}", step + 1);

        // Initial commitment
        println!("Client phase");
        let (random_value, commitment) = client.generate_commitment();
        println!("Initial commitment: {}", commitment);

        // Authentication challenge
        println!("Authenticator phase");
        let challenge_bits = auth.generate_challenge();
        println!("Challenge vector: {:?}", challenge_bits);

        // Client response
        println!("Client phase");
        let proof = client.compute_proof(&random_value, &challenge_bits);
        println!("Proof value: {}", proof);

        // Verification
        println!("Authenticator phase");
        let is_valid = auth.validate_proof(&commitment, &proof, &challenge_bits);
        println!("Validation result: {}", is_valid);

        if !is_valid {
            return false;
        }
    }

    true
}

fn main() {
    let result = authentication_protocol(50);
    assert!(result);
}