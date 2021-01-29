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
    fn test(n: &BigUint, rounds: u32) -> Primality;
}

mod fermat;
mod millerrabin;
mod solovaystrassen;

pub use fermat::Fermat;
pub use millerrabin::MillerRabin;
pub use solovaystrassen::SolovayStrassen;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Tests all algorithms with 300 randomly generated 1024-bit integers.
    fn test_all() {
        let mut rng = thread_rng();

        for _ in 0..300 {
            let test = rng.gen_biguint(1024);
            let mr = MillerRabin::test(&test, 5);
            let sostr = SolovayStrassen::test(&test, 5);
            let f = Fermat::test(&test, 5);
            assert_eq!(mr, sostr);
            assert_eq!(mr, f);
        }
    }
}
