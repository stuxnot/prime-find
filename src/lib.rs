extern crate num_bigint;
extern crate rand;
use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

#[derive(PartialEq, Eq, Debug)]
pub enum Primality {
    Composite,
    ProbablyPrime,
}

pub trait ProbabilisticPrimalityTester {
    /// Tests if a number is prime using a probabilistic method.
    /// The given number n should be greater than 3 and odd.
    /// `rounds` specifies the number of tests that are performed.
    /// A higher number of rounds reduces the probability of a false
    /// positive.
    fn test(n: &BigUint, rounds: u32) -> Primality;
}

mod fermat;
mod millerrabin;
mod solovaystrassen;
mod trial;

pub use fermat::Fermat;
pub use millerrabin::MillerRabin;
pub use solovaystrassen::SolovayStrassen;
pub use trial::prime_test_trial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Tests all algorithms with 100 randomly generated 256-bit integers.
    fn random_numbers() {
        let mut rng = thread_rng();

        for _ in 0..200 {
            let mut test = rng.gen_biguint(256);

            while &test < &4u8.into() {
                test = rng.gen_biguint(256);
            }

            test.set_bit(0, true); // Assure that test ist odd

            let mr = MillerRabin::test(&test, 3);
            let sostr = SolovayStrassen::test(&test, 3);
            let f = Fermat::test(&test, 3);
            assert_eq!(mr, sostr);
            assert_eq!(mr, f);
        }
    }

    #[test]
    fn find_2048_bit_prime() {
        let mut rng = thread_rng();

        let mut count = 0u32;
        let mut passed_trial = 0u32;

        let mut prime;
        loop {
            count += 1;

            prime = rng.gen_biguint(2048);
            prime.set_bit(0, true);

            if prime_test_trial(&prime) == Primality::Composite {
                continue;
            }

            passed_trial += 1;

            if Fermat::test(&prime, 2) == Primality::ProbablyPrime {
                if MillerRabin::test(&prime, 3) == Primality::ProbablyPrime {
                    break;
                }
            }
        }

        println!(
            "Tested {} numbers. {} % passed the trial",
            count,
            passed_trial * 100 / count
        );

        println!("Prime:\n {}", prime);
    }
}
