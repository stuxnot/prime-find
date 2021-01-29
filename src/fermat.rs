use super::thread_rng;
use super::{BigUint, RandBigInt};
use super::{Primality, ProbabilisticPrimalityTester};

pub struct Fermat;

impl ProbabilisticPrimalityTester for Fermat {
    fn test(n: &BigUint, rounds: u32) -> Primality {
        let one_const = BigUint::from(1u8);
        let two_const = BigUint::from(2u8);
        let three_const = BigUint::from(3u8);

        if n < &two_const {
            return Primality::Composite;
        }

        // We cannot generate a number between [2, 3 - 2] or [2, 2 - 2]
        if n <= &three_const {
            return Primality::ProbablyPrime;
        }

        // If the number is even, we know it is not a prime.
        if n & (&one_const) != one_const {
            return Primality::Composite;
        }

        let mut rng = thread_rng();
        let n_decrement = n - 1u8;
        for _ in 0..rounds {
            let a = rng.gen_biguint_range(&two_const, &n_decrement);

            if a.modpow(&n_decrement, n) != one_const {
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
    fn test_fermat() {
        for i in 0u8..10u8 {
            match i {
                2 | 3 | 5 | 7 | 11 => {
                    assert_eq!(Fermat::test(&BigUint::from(i), 2), Primality::ProbablyPrime)
                }
                _ => assert_eq!(Fermat::test(&BigUint::from(i), 2), Primality::Composite),
            }
        }
    }
}
