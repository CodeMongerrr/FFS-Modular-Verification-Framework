use num_bigint::BigUint;
use rand::Rng;

pub struct Authenticator {
    modulus: BigUint,
    public_keys: Vec<BigUint>,
}

impl Authenticator {
    pub fn initialize(modulus: BigUint, public_keys: Vec<BigUint>) -> Self {
        Authenticator { modulus, public_keys }
    }

    pub fn generate_challenge(&self) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        self.public_keys.iter().map(|_| rng.gen_range(0..2)).collect()
    }

    pub fn validate_proof(&self, commitment: &BigUint, proof: &BigUint, challenge: &[u32]) -> bool {
        if commitment >= &self.modulus || commitment <= &BigUint::from(0_u32) {
            panic!("Invalid commitment value");
        }

        let mut computed_value = BigUint::from(1_u32);
        for (idx, &bit) in challenge.iter().enumerate() {
            if bit == 1 {
                computed_value *= self.public_keys[idx].modpow(&BigUint::from(bit), &self.modulus);
                computed_value %= &self.modulus;
            }
        }

        let squared_proof = proof.modpow(&BigUint::from(2_u32), &self.modulus);
        let option1 = &computed_value * commitment % &self.modulus;
        let option2 = &computed_value * (&self.modulus - commitment) % &self.modulus;

        squared_proof == option1 || squared_proof == option2
    }
}