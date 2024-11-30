use num_bigint::BigUint;
use rand::Rng;

#[derive(Debug)]
pub struct Client {
    pub modulus: BigUint,
    secrets: Vec<BigUint>,
    pub public_keys: Vec<BigUint>,
}

impl Client {
    pub fn initialize(p1: BigUint, p2: BigUint, secrets: Vec<BigUint>) -> Self {
        let modulus = &p1 * &p2;
        let public_keys: Vec<BigUint> = secrets
            .iter()
            .map(|secret| secret.modpow(&BigUint::from(2_u32), &modulus))
            .collect();
        
        Client { modulus, secrets, public_keys }
    }

    fn generate_random_bit() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..2) * 2 - 1
    }

    pub fn generate_commitment(&self) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let random_val: BigUint = rng.sample(num_bigint::RandomBits::new(256));
        let bit = Self::generate_random_bit();
        
        match bit {
            1 => {
                let commitment = random_val.modpow(&BigUint::from(2_u32), &self.modulus);
                (random_val, commitment)
            }
            -1 => {
                let squared = random_val.modpow(&BigUint::from(2_u32), &self.modulus);
                let commitment = &self.modulus - &squared;
                (random_val, commitment)
            }
            _ => panic!("Unexpected bit value"),
        }
    }

    pub fn compute_proof(&self, random_val: &BigUint, challenge: &[u32]) -> BigUint {
        let mut result = BigUint::from(1_u32);
        result *= random_val;
        
        for (idx, &bit) in challenge.iter().enumerate() {
            result *= self.secrets[idx].modpow(&BigUint::from(bit), &self.modulus);
            result %= &self.modulus;
        }
        
        result
    }
}