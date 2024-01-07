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
}
