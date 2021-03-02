use super::thread_rng;
use super::{BigUint, RandBigInt};
use super::{Primality, ProbabilisticPrimalityTester};

pub struct SolovayStrassen;

fn jacobi_symbol(a: &BigUint, n: &BigUint) -> Option<i8> {
    // The LSB must be one.
    if n.trailing_zeros() != Some(0) {
        return None;
    }

    let mut a = a.clone();
    let mut n = n.clone();

    a = a % &n;

    let mut t: i8 = 1;

    let zero_const = BigUint::from(0u8);
    let three_const = BigUint::from(3u8);
    let five_const = BigUint::from(5u8);

    while a != zero_const {
        if let Some(num) = a.trailing_zeros() {
            for _ in 0..num {
                a >>= 1;
                let r = &n % 8u8;
                if r == three_const || r == five_const {
                    t = -t;
                }
            }
        }

        std::mem::swap(&mut a, &mut n);

        if &a % 4u8 == three_const && &n % 4u8 == three_const {
            t = -t;
        }

        a = a % &n;
    }

    if n == BigUint::from(1u8) {
        return Some(t);
    }

    Some(0)
}

impl ProbabilisticPrimalityTester for SolovayStrassen {
    fn test(n: &BigUint, rounds: u32) -> Primality {
        let one_const = BigUint::from(1u8);
        let two_const = BigUint::from(2u8);

        if n < &two_const {
            return Primality::Composite;
        }

        // We cannot generate a number between [2, 2 - 1]
        if n == &two_const {
            return Primality::ProbablyPrime;
        }

        // If the number is even, we know it is not a prime.
        if n & (&one_const) != one_const {
            return Primality::Composite;
        }

        let mut rng = thread_rng();

        let legendre_power = (n - 1u8) >> 1;

        for _ in 0..rounds {
            let a = rng.gen_biguint_range(&two_const, n);
            // Safety: We can unwrap the value here, since we checked earlier that n is odd.
            let x = jacobi_symbol(&a, n).unwrap();

            let x = match x.signum() {
                0 => return Primality::Composite,
                -1 => (n - (-x as u8)) % n,
                1 => (n + (x as u8)) % n,
                _ => unreachable!(),
            };

            if a.modpow(&legendre_power, n) != x {
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
    fn test_jacobi() {
        // Tests some known jacobi symbol values
        assert_eq!(
            jacobi_symbol(&BigUint::from(1u8), &BigUint::from(1u8)),
            Some(1)
        );
        assert_eq!(
            jacobi_symbol(&BigUint::from(2u8), &BigUint::from(3u8)),
            Some(-1)
        );
        assert_eq!(
            jacobi_symbol(&BigUint::from(5u8), &BigUint::from(51u8)),
            Some(1)
        );
        assert_eq!(
            jacobi_symbol(&BigUint::from(21u8), &BigUint::from(53u8)),
            Some(-1)
        );
        assert_eq!(
            jacobi_symbol(&BigUint::from(6u8), &BigUint::from(21u8)),
            Some(0)
        );
    }

    #[test]
    fn test_solovay_strassen() {
        for i in 0u8..11u8 {
            match i {
                2 | 3 | 5 | 7 | 11 => assert_eq!(
                    SolovayStrassen::test(&BigUint::from(i), 2),
                    Primality::ProbablyPrime
                ),
                _ => assert_eq!(
                    SolovayStrassen::test(&BigUint::from(i), 2),
                    Primality::Composite
                ),
            }
        }
    }
}
