use super::thread_rng;
use super::{BigUint, RandBigInt};
use super::{Primality, ProbabilisticPrimalityTester};

pub struct MillerRabin;

impl ProbabilisticPrimalityTester for MillerRabin {
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

        let n_decrement: BigUint = n - 1u8;
        let mut d = n_decrement.clone();
        let mut r: u64 = 0;
        if let Some(trail) = d.trailing_zeros() {
            d >>= trail;
            r += trail;
        }

        let mut rng = thread_rng();

        'witnessLoop: for _ in 0..rounds {
            let a = rng.gen_biguint_range(&two_const, &n_decrement);
            let mut x = a.modpow(&d, n);
            assert!(&x < n);

            if x == one_const || x == n_decrement {
                continue 'witnessLoop;
            }

            for _ in 0..(r - 1) {
                x = x.modpow(&two_const, n);
                assert!(&x < n);
                if x == n_decrement {
                    continue 'witnessLoop;
                }
            }

            return Primality::Composite;
        }

        Primality::ProbablyPrime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin() {
        for i in 0u8..11u8 {
            match i {
                2 | 3 | 5 | 7 | 11 => assert_eq!(
                    MillerRabin::test(&BigUint::from(i), 2),
                    Primality::ProbablyPrime
                ),
                _ => assert_eq!(
                    MillerRabin::test(&BigUint::from(i), 2),
                    Primality::Composite
                ),
            }
        }
    }
}
