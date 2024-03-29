use super::thread_rng;
use super::{BigUint, RandBigInt};
use super::{Primality, ProbabilisticPrimalityTester};

pub struct Fermat;

impl ProbabilisticPrimalityTester for Fermat {
    fn test(n: &BigUint, rounds: u32) -> Primality {
        // Assumes n > 3, odd
        let mut rng = thread_rng();
        let n_decrement = n - 1u8;
        for _ in 0..rounds {
            let a = rng.gen_biguint_range(&(2u8.into()), &n_decrement);

            if a.modpow(&n_decrement, n) != 1u8.into() {
                return Primality::Composite;
            }
        }

        Primality::ProbablyPrime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_prime() {
        const LARGE_PRIMES: [&str; 1] = ["359334085968622831041960188598043661065388726959079837"];

        for prime in LARGE_PRIMES {
            let prime = BigUint::parse_bytes(prime.as_bytes(), 10).unwrap();
            assert_eq!(Fermat::test(&prime, 2), Primality::ProbablyPrime);
        }
    }

    #[test]
    fn small_primes() {
        const SMALL_PRIMES: [u32; 52] = [
            0x05, 0x07, 0x0B, 0x0D, 0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25, 0x29, 0x2B, 0x2F, 0x35,
            0x3B, 0x3D, 0x43, 0x47, 0x49, 0x4F, 0x53, 0x59, 0x61, 0x65, 0x67, 0x6B, 0x6D, 0x71,
            0x7F, 0x83, 0x89, 0x8B, 0x95, 0x97, 0x9D, 0xA3, 0xA7, 0xAD, 0xB3, 0xB5, 0xBF, 0xC1,
            0xC5, 0xC7, 0xD3, 0xDF, 0xE3, 0xE5, 0xE9, 0xEF, 0xF1, 0xFB,
        ];

        for prime in SMALL_PRIMES {
            assert_eq!(Fermat::test(&prime.into(), 2), Primality::ProbablyPrime);
        }
    }

    #[test]
    fn test_non_prime() {
        const SOME_NON_PRIMES: [&str; 2] = ["25", "235"];
        for non_prime in SOME_NON_PRIMES {
            let non_prime = BigUint::parse_bytes(non_prime.as_bytes(), 10).unwrap();
            assert_eq!(Fermat::test(&non_prime, 2), Primality::Composite);
        }
    }
}
